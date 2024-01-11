
//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-08, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-hetzner-01`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/kintsugi/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {

	/// Storage: Scheduler IncompleteSince (r:1 w:1)
	/// Proof: Scheduler IncompleteSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_agendas_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `1489`
		// Minimum execution time: 6_693_000 picoseconds.
		Weight::from_parts(6_814_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(23383), added: 25858, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 30]`.
	fn service_agenda_base	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + s * (177 ±0)`
		//  Estimated: `26848`
		// Minimum execution time: 6_774_000 picoseconds.
		Weight::from_parts(11_503_134, 26848)
			// Standard Error: 7_777
			.saturating_add(Weight::from_parts(1_401_937, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_924_000 picoseconds.
		Weight::from_parts(12_104_000, 0)
	}
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `179 + s * (1 ±0)`
		//  Estimated: `3644 + s * (1 ±0)`
		// Minimum execution time: 38_106_000 picoseconds.
		Weight::from_parts(38_387_000, 3644)
			// Standard Error: 32
			.saturating_add(Weight::from_parts(1_599, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
	}
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn service_task_named	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_160_000 picoseconds.
		Weight::from_parts(16_062_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_periodic	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_794_000 picoseconds.
		Weight::from_parts(12_094_000, 0)
	}
	fn execute_dispatch_signed	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_182_000 picoseconds.
		Weight::from_parts(6_503_000, 0)
	}
	fn execute_dispatch_unsigned	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_233_000 picoseconds.
		Weight::from_parts(6_313_000, 0)
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(23383), added: 25858, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 29]`.
	fn schedule	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + s * (177 ±0)`
		//  Estimated: `26848`
		// Minimum execution time: 26_824_000 picoseconds.
		Weight::from_parts(32_249_971, 26848)
			// Standard Error: 9_995
			.saturating_add(Weight::from_parts(1_305_260, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(23383), added: 25858, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 30]`.
	fn cancel	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + s * (177 ±0)`
		//  Estimated: `26848`
		// Minimum execution time: 32_946_000 picoseconds.
		Weight::from_parts(31_457_391, 26848)
			// Standard Error: 7_426
			.saturating_add(Weight::from_parts(2_440_780, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(23383), added: 25858, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 29]`.
	fn schedule_named	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `186 + s * (189 ±0)`
		//  Estimated: `26848`
		// Minimum execution time: 32_335_000 picoseconds.
		Weight::from_parts(37_831_818, 26848)
			// Standard Error: 13_013
			.saturating_add(Weight::from_parts(1_437_155, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(23383), added: 25858, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 30]`.
	fn cancel_named	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217 + s * (188 ±0)`
		//  Estimated: `26848`
		// Minimum execution time: 35_511_000 picoseconds.
		Weight::from_parts(35_050_635, 26848)
			// Standard Error: 8_981
			.saturating_add(Weight::from_parts(2_484_597, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}