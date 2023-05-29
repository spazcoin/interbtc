
//! Autogenerated weights for farming
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-04, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `enterprise`, CPU: `Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// farming
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 2
// --repeat
// 1
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for farming using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> farming::WeightInfo for WeightInfo<T> {

	/// Storage: Farming RewardSchedules (r:5 w:0)
	/// Proof: Farming RewardSchedules (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: FarmingRewards TotalStake (r:2 w:0)
	/// Proof: FarmingRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 4]`.
	fn on_initialize	(_c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1153 + c * (38 ±0)`
		//  Estimated: `17781`
		// Minimum execution time: 41_211_000 picoseconds.
		Weight::from_parts(68_374_000, 17781)
			.saturating_add(T::DbWeight::get().reads(7_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Farming RewardSchedules (r:1 w:1)
	/// Proof: Farming RewardSchedules (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn update_reward_schedule	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1705`
		//  Estimated: `10332`
		// Minimum execution time: 84_033_000 picoseconds.
		Weight::from_parts(84_033_000, 10332)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Farming RewardSchedules (r:0 w:1)
	/// Proof: Farming RewardSchedules (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn remove_reward_schedule	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1755`
		//  Estimated: `7783`
		// Minimum execution time: 70_634_000 picoseconds.
		Weight::from_parts(70_634_000, 7783)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: FarmingRewards RewardCurrencies (r:1 w:0)
	/// Proof: FarmingRewards RewardCurrencies (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: FarmingRewards Stake (r:1 w:1)
	/// Proof: FarmingRewards Stake (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	/// Storage: FarmingRewards TotalStake (r:1 w:1)
	/// Proof: FarmingRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardTally (r:4 w:4)
	/// Proof: FarmingRewards RewardTally (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardPerToken (r:4 w:0)
	/// Proof: FarmingRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 4]`.
	fn deposit	(_c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1702 + c * (70 ±0)`
		//  Estimated: `30759`
		// Minimum execution time: 79_837_000 picoseconds.
		Weight::from_parts(109_013_000, 30759)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: FarmingRewards RewardCurrencies (r:1 w:0)
	/// Proof: FarmingRewards RewardCurrencies (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: FarmingRewards Stake (r:1 w:1)
	/// Proof: FarmingRewards Stake (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	/// Storage: FarmingRewards TotalStake (r:1 w:1)
	/// Proof: FarmingRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardTally (r:4 w:4)
	/// Proof: FarmingRewards RewardTally (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardPerToken (r:4 w:0)
	/// Proof: FarmingRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 4]`.
	fn withdraw	(_c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1702 + c * (70 ±0)`
		//  Estimated: `30759`
		// Minimum execution time: 76_083_000 picoseconds.
		Weight::from_parts(102_982_000, 30759)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: FarmingRewards Stake (r:1 w:0)
	/// Proof: FarmingRewards Stake (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardPerToken (r:1 w:0)
	/// Proof: FarmingRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: FarmingRewards RewardTally (r:1 w:1)
	/// Proof: FarmingRewards RewardTally (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: FarmingRewards TotalRewards (r:1 w:1)
	/// Proof: FarmingRewards TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2228`
		//  Estimated: `17973`
		// Minimum execution time: 104_159_000 picoseconds.
		Weight::from_parts(104_159_000, 17973)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}