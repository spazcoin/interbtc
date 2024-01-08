
//! Autogenerated weights for orml_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-08, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-hetzner-01`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for orml_vesting using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> orml_vesting::WeightInfo for WeightInfo<T> {

	/// Storage: Vesting VestingSchedules (r:1 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1]`.
	fn claim	(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771 + n * (109 ±0)`
		//  Estimated: `4733`
		// Minimum execution time: 68_938_000 picoseconds.
		Weight::from_parts(86_073_760, 4733)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Vesting VestingSchedules (r:1 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	fn vested_transfer	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `851`
		//  Estimated: `6170`
		// Minimum execution time: 133_588_000 picoseconds.
		Weight::from_parts(134_880_000, 6170)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1268), added: 3743, mode: MaxEncodedLen)
	/// Storage: Vesting VestingSchedules (r:0 w:1)
	/// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(77), added: 2552, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1]`.
	fn update_vesting_schedules	(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `894`
		//  Estimated: `4733`
		// Minimum execution time: 72_215_000 picoseconds.
		Weight::from_parts(76_821_817, 4733)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}