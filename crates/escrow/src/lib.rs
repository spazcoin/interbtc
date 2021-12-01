//! # Escrow Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The escrow pallet allows accounts to lock the native currency and receive vote-escrowed tokens.
//! This voting power linearly decreases per block and tends toward zero as the height approaches
//! the max lockup period.
//!
//! This implementation is based in part on Curve's implementation, but explicitly follows
//! the specification at <https://spec.interlay.io/spec/escrow.html>.

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::{
    ensure,
    traits::{
        BalanceStatus, Currency, ExistenceRequirement, Get, Imbalance, LockIdentifier, LockableCurrency,
        ReservableCurrency, SignedImbalance, WithdrawReasons,
    },
    transactional,
};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, CheckedSub, Convert, Saturating, Zero},
    DispatchError, DispatchResult,
};

const LOCK_ID: LockIdentifier = *b"escrowed";

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type PositiveImbalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
type NegativeImbalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

#[derive(Default, Encode, Decode, Debug, Clone, TypeInfo)]
pub struct Point<Balance, BlockNumber> {
    bias: Balance,
    slope: Balance,
    ts: BlockNumber,
}

impl<Balance: AtLeast32BitUnsigned + Copy, BlockNumber: AtLeast32BitUnsigned + Copy> Point<Balance, BlockNumber> {
    fn new<BlockNumberToBalance: Convert<BlockNumber, Balance>>(
        amount: Balance,
        start_height: BlockNumber,
        end_height: BlockNumber,
        max_period: BlockNumber,
    ) -> Self {
        let max_period = BlockNumberToBalance::convert(max_period);
        let height_diff = BlockNumberToBalance::convert(end_height.saturating_sub(start_height));

        let slope = amount / max_period;
        let bias = slope * height_diff;

        Self {
            bias,
            slope,
            ts: start_height,
        }
    }

    fn balance_at<BlockNumberToBalance: Convert<BlockNumber, Balance>>(&self, height: BlockNumber) -> Balance {
        let height_diff = BlockNumberToBalance::convert(height.saturating_sub(self.ts));
        self.bias.saturating_sub(self.slope.saturating_mul(height_diff))
    }
}

#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct LockedBalance<Balance, BlockNumber> {
    amount: Balance,
    end: BlockNumber,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// ## Configuration
    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Convert the block number into a balance.
        type BlockNumberToBalance: Convert<Self::BlockNumber, BalanceOf<Self>>;

        /// The currency trait.
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>
            + ReservableCurrency<Self::AccountId>;

        /// All future times are rounded by this.
        #[pallet::constant]
        type Span: Get<Self::BlockNumber>;

        /// The maximum time for locks.
        #[pallet::constant]
        type MaxPeriod: Get<Self::BlockNumber>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    // The pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        Deposit {
            who: T::AccountId,
            amount: BalanceOf<T>,
            unlock_height: T::BlockNumber,
        },
        Withdraw {
            who: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidAmount,
        InvalidHeight,
        LockNotFound,
        LockNotExpired,
        LockHasExpired,
        InsufficientFunds,
        InvalidAction,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::storage]
    #[pallet::getter(fn reserved_balance)]
    pub type Reserved<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn locked_balance)]
    pub type Locked<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, LockedBalance<BalanceOf<T>, T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    pub type Epoch<T: Config> = StorageValue<_, T::Index, ValueQuery>;

    #[pallet::storage]
    pub type PointHistory<T: Config> =
        StorageMap<_, Identity, T::Index, Point<BalanceOf<T>, T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    pub type UserPointHistory<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Identity,
        T::Index,
        Point<BalanceOf<T>, T::BlockNumber>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type UserPointEpoch<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Index, ValueQuery>;

    #[pallet::storage]
    pub type SlopeChanges<T: Config> = StorageMap<_, Blake2_128Concat, T::BlockNumber, BalanceOf<T>, ValueQuery>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::create_lock())]
        #[transactional]
        pub fn create_lock(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
            unlock_height: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = Self::current_height();

            // lock time is rounded down to weeks
            let unlock_height = Self::round_height(unlock_height);

            // value MUST be non-zero
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            // user MUST withdraw first
            ensure!(Self::locked_balance(&who).amount.is_zero(), Error::<T>::InvalidAmount);

            // unlock MUST be in the future
            ensure!(unlock_height > now, Error::<T>::InvalidHeight);

            // height MUST NOT be greater than max
            let max_period = T::MaxPeriod::get();
            let end_height = now.saturating_add(max_period);
            ensure!(unlock_height <= end_height, Error::<T>::InvalidHeight);

            Self::deposit_for(&who, amount, unlock_height)?;

            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::increase_amount())]
        #[transactional]
        pub fn increase_amount(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let locked_balance = Self::locked_balance(&who);
            let now = Self::current_height();

            // value MUST be non-zero
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            // lock MUST exist first
            ensure!(!locked_balance.amount.is_zero(), Error::<T>::LockNotFound);

            // lock MUST NOT be expired
            ensure!(locked_balance.end > now, Error::<T>::LockHasExpired);

            Self::deposit_for(&who, amount, Zero::zero())?;

            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::increase_unlock_height())]
        #[transactional]
        pub fn increase_unlock_height(
            origin: OriginFor<T>,
            unlock_height: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let locked_balance = Self::locked_balance(&who);
            let now = Self::current_height();

            // lock time is rounded down to weeks
            let unlock_height = Self::round_height(unlock_height);

            // lock MUST NOT be expired
            ensure!(locked_balance.end > now, Error::<T>::LockHasExpired);

            // lock amount MUST be non-zero
            ensure!(!locked_balance.amount.is_zero(), Error::<T>::InvalidAmount);

            // lock duration MUST increase
            ensure!(unlock_height > locked_balance.end, Error::<T>::InvalidHeight);

            // height MUST NOT be greater than max
            let max_period = T::MaxPeriod::get();
            let end_height = now.saturating_add(max_period);
            ensure!(unlock_height <= end_height, Error::<T>::InvalidHeight);

            Self::deposit_for(&who, Zero::zero(), unlock_height)?;

            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::withdraw())]
        #[transactional]
        pub fn withdraw(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::remove_lock(&who)?;
            Ok(().into())
        }
    }
}

type DefaultPoint<T> = Point<BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;
type DefaultLockedBalance<T> = LockedBalance<BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

// "Internal" functions, callable by code.
impl<T: Config> Pallet<T> {
    fn current_height() -> T::BlockNumber {
        frame_system::Pallet::<T>::block_number()
    }

    fn round_height(height: T::BlockNumber) -> T::BlockNumber {
        let span = T::Span::get();
        (height / span) * span
    }

    fn checkpoint(who: &T::AccountId, old_locked: DefaultLockedBalance<T>, new_locked: DefaultLockedBalance<T>) {
        let now = Self::current_height();
        let max_period = T::MaxPeriod::get();

        let u_old = if old_locked.end > now && old_locked.amount > Zero::zero() {
            Point::new::<T::BlockNumberToBalance>(old_locked.amount, now, old_locked.end, max_period)
        } else {
            Default::default()
        };

        let u_new = if new_locked.end > now && new_locked.amount > Zero::zero() {
            Point::new::<T::BlockNumberToBalance>(new_locked.amount, now, new_locked.end, max_period)
        } else {
            Default::default()
        };

        let mut old_dslope = <SlopeChanges<T>>::get(old_locked.end);
        let mut new_dslope = if !new_locked.end.is_zero() {
            if new_locked.end == old_locked.end {
                old_dslope
            } else {
                <SlopeChanges<T>>::get(new_locked.end)
            }
        } else {
            Zero::zero()
        };

        let mut epoch = <Epoch<T>>::get();
        let mut last_point = <PointHistory<T>>::get(epoch);
        let mut last_checkpoint = last_point.ts;

        let mut t_i = Self::round_height(last_checkpoint);
        while t_i < now {
            t_i.saturating_accrue(T::Span::get());
            let d_slope = if t_i > now {
                t_i = now;
                Zero::zero()
            } else {
                <SlopeChanges<T>>::get(t_i)
            };
            let height_diff = T::BlockNumberToBalance::convert(t_i.saturating_sub(last_checkpoint));
            last_point.bias.saturating_reduce(last_point.slope * height_diff);
            last_point.slope.saturating_accrue(d_slope);
            last_checkpoint = t_i;
            last_point.ts = t_i;
            epoch.saturating_inc();

            if t_i == now {
                break;
            }

            <PointHistory<T>>::insert(epoch, last_point.clone());
        }

        <Epoch<T>>::put(epoch);

        last_point
            .slope
            .saturating_accrue(u_new.slope.saturating_sub(u_old.slope));
        last_point.bias.saturating_accrue(u_new.bias.saturating_sub(u_old.bias));
        <PointHistory<T>>::insert(epoch, last_point);

        if old_locked.end > now {
            old_dslope.saturating_accrue(u_old.slope);
            if new_locked.end == old_locked.end {
                // new deposit
                old_dslope = old_dslope.saturating_sub(u_new.slope);
            }
            <SlopeChanges<T>>::insert(old_locked.end, old_dslope);
        }

        if new_locked.end > now && new_locked.end > old_locked.end {
            new_dslope = new_dslope.saturating_sub(u_new.slope);
            <SlopeChanges<T>>::insert(new_locked.end, new_dslope);
        }

        let user_epoch = <UserPointEpoch<T>>::mutate(who, |i| {
            i.saturating_inc();
            *i
        });
        <UserPointHistory<T>>::insert(who, user_epoch, u_new);
    }

    fn deposit_for(who: &T::AccountId, amount: BalanceOf<T>, unlock_height: T::BlockNumber) -> DispatchResult {
        let old_locked = Self::locked_balance(who);
        let mut new_locked = old_locked.clone();
        new_locked.amount += amount;
        if unlock_height > Zero::zero() {
            new_locked.end = unlock_height;
        }

        ensure!(
            T::Currency::free_balance(who) >= new_locked.amount,
            Error::<T>::InsufficientFunds,
        );
        T::Currency::set_lock(LOCK_ID, &who, new_locked.amount, WithdrawReasons::all());
        <Locked<T>>::insert(who, new_locked.clone());

        Self::checkpoint(who, old_locked, new_locked);

        Self::deposit_event(Event::<T>::Deposit {
            who: who.clone(),
            amount,
            unlock_height,
        });

        Ok(())
    }

    fn remove_lock(who: &T::AccountId) -> DispatchResult {
        let old_locked = <Locked<T>>::take(who);
        let amount = old_locked.amount;
        let current_height = Self::current_height();

        // lock MUST have expired
        ensure!(current_height >= old_locked.end, Error::<T>::LockNotExpired);

        Self::checkpoint(who, old_locked, Default::default());

        T::Currency::remove_lock(LOCK_ID, &who);
        <UserPointHistory<T>>::remove_prefix(who, None);

        Self::deposit_event(Event::<T>::Withdraw {
            who: who.clone(),
            amount,
        });

        Ok(())
    }

    pub fn balance_at(who: &T::AccountId, height: Option<T::BlockNumber>) -> BalanceOf<T> {
        let height = height.unwrap_or(Self::current_height());
        let last_point = <UserPointHistory<T>>::get(who, <UserPointEpoch<T>>::get(who));
        last_point.balance_at::<T::BlockNumberToBalance>(height)
    }

    pub fn supply_at(point: DefaultPoint<T>, height: T::BlockNumber) -> BalanceOf<T> {
        let mut last_point = point;

        let mut t_i = Self::round_height(last_point.ts);
        while t_i < height {
            t_i.saturating_accrue(T::Span::get());

            let d_slope = if t_i > height {
                t_i = height;
                Zero::zero()
            } else {
                <SlopeChanges<T>>::get(t_i)
            };

            let height_diff = T::BlockNumberToBalance::convert(t_i.saturating_sub(last_point.ts));
            last_point.bias.saturating_reduce(last_point.slope * height_diff);

            if t_i == height {
                break;
            }

            last_point.slope.saturating_accrue(d_slope);
            last_point.ts = t_i;
        }

        last_point.bias
    }

    pub fn total_supply(height: Option<T::BlockNumber>) -> BalanceOf<T> {
        let height = height.unwrap_or(Self::current_height());
        let last_point = <PointHistory<T>>::get(<Epoch<T>>::get());
        Self::supply_at(last_point, height)
    }
}

impl<T: Config> Currency<T::AccountId> for Pallet<T> {
    type Balance = BalanceOf<T>;
    type PositiveImbalance = PositiveImbalanceOf<T>;
    type NegativeImbalance = NegativeImbalanceOf<T>;

    fn total_balance(who: &T::AccountId) -> Self::Balance {
        Pallet::<T>::balance_at(who, None)
    }

    // NOT SUPPORTED
    fn can_slash(_who: &T::AccountId, _value: Self::Balance) -> bool {
        false
    }

    fn total_issuance() -> Self::Balance {
        Pallet::<T>::total_supply(None)
    }

    fn minimum_balance() -> Self::Balance {
        T::Currency::minimum_balance()
    }

    // NOT SUPPORTED
    fn burn(_amount: Self::Balance) -> Self::PositiveImbalance {
        Imbalance::zero()
    }

    // NOT SUPPORTED
    fn issue(_amount: Self::Balance) -> Self::NegativeImbalance {
        Imbalance::zero()
    }

    fn free_balance(who: &T::AccountId) -> Self::Balance {
        Pallet::<T>::balance_at(who, None).saturating_sub(Pallet::<T>::reserved_balance(who))
    }

    // NOT SUPPORTED
    fn ensure_can_withdraw(
        _who: &T::AccountId,
        _amount: Self::Balance,
        _reasons: WithdrawReasons,
        _new_balance: Self::Balance,
    ) -> DispatchResult {
        Err(Error::<T>::InvalidAction.into())
    }

    // NOT SUPPORTED
    fn transfer(
        _source: &T::AccountId,
        _dest: &T::AccountId,
        _value: Self::Balance,
        _existence_requirement: ExistenceRequirement,
    ) -> DispatchResult {
        Err(Error::<T>::InvalidAction.into())
    }

    // NOT SUPPORTED
    fn slash(_who: &T::AccountId, _value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        (Imbalance::zero(), Zero::zero())
    }

    // NOT SUPPORTED
    fn deposit_into_existing(
        _who: &T::AccountId,
        _value: Self::Balance,
    ) -> sp_std::result::Result<Self::PositiveImbalance, DispatchError> {
        Err(Error::<T>::InvalidAction.into())
    }

    // NOT SUPPORTED
    fn deposit_creating(_who: &T::AccountId, _value: Self::Balance) -> Self::PositiveImbalance {
        Imbalance::zero()
    }

    // NOT SUPPORTED
    fn withdraw(
        _who: &T::AccountId,
        _value: Self::Balance,
        _reasons: WithdrawReasons,
        _liveness: ExistenceRequirement,
    ) -> sp_std::result::Result<Self::NegativeImbalance, DispatchError> {
        Err(Error::<T>::InvalidAction.into())
    }

    fn make_free_balance_be(
        who: &T::AccountId,
        balance: Self::Balance,
    ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        T::Currency::make_free_balance_be(who, balance)
    }
}

impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T> {
    fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
        Pallet::<T>::free_balance(who).checked_sub(&value).is_some()
    }

    // NOT SUPPORTED
    fn slash_reserved(_who: &T::AccountId, _value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        (Imbalance::zero(), Zero::zero())
    }

    fn reserved_balance(who: &T::AccountId) -> Self::Balance {
        Pallet::<T>::reserved_balance(who)
    }

    fn reserve(who: &T::AccountId, value: Self::Balance) -> DispatchResult {
        if !Pallet::<T>::can_reserve(who, value) {
            return Err(Error::<T>::InsufficientFunds.into());
        }
        <Reserved<T>>::mutate(who, |previous| previous.saturating_accrue(value));
        Ok(())
    }

    fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        <Reserved<T>>::mutate(who, |previous| {
            if value > *previous {
                let remainder = value - *previous;
                *previous = Zero::zero();
                remainder
            } else {
                *previous -= value;
                Zero::zero()
            }
        })
    }

    // NOT SUPPORTED
    fn repatriate_reserved(
        _slashed: &T::AccountId,
        _beneficiary: &T::AccountId,
        _value: Self::Balance,
        _status: BalanceStatus,
    ) -> sp_std::result::Result<Self::Balance, DispatchError> {
        Err(Error::<T>::InvalidAction.into())
    }
}