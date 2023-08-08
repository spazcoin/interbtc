
//! Autogenerated weights for dex_stable
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-07, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-rust-runner-2mz2v-kcxvd`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
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

/// Weights for dex_stable using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> dex_stable::WeightInfo for WeightInfo<T> {

	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DexStable LpCurrencies (r:1 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_base_pool	(b: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227`
		//  Estimated: `4281`
		// Minimum execution time: 55_992_000 picoseconds.
		Weight::from_parts(56_658_626, 4281)
			// Standard Error: 8_375
			.saturating_add(Weight::from_parts(154_956, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable LpCurrencies (r:2 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:2 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_meta_pool	(m: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1426`
		//  Estimated: `7572`
		// Minimum execution time: 104_991_000 picoseconds.
		Weight::from_parts(106_170_089, 7572)
			// Standard Error: 150_554
			.saturating_add(Weight::from_parts(92_297, 0).saturating_mul(m.into()))
			// Standard Error: 25_055
			.saturating_add(Weight::from_parts(69_010, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn add_liquidity	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1264 + b * (125 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 196_874_000 picoseconds.
		Weight::from_parts(98_585_099, 4281)
			// Standard Error: 77_387
			.saturating_add(Weight::from_parts(50_481_828, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2366`
		//  Estimated: `11350`
		// Minimum execution time: 155_441_000 picoseconds.
		Weight::from_parts(157_406_000, 11350)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1315 + b * (192 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 152_626_000 picoseconds.
		Weight::from_parts(84_999_040, 4281)
			// Standard Error: 67_946
			.saturating_add(Weight::from_parts(34_274_766, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity_one_currency	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2465`
		//  Estimated: `8760`
		// Minimum execution time: 178_487_000 picoseconds.
		Weight::from_parts(180_611_000, 8760)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity_imbalance	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1357 + b * (192 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 177_556_000 picoseconds.
		Weight::from_parts(99_852_974, 4281)
			// Standard Error: 65_802
			.saturating_add(Weight::from_parts(39_938_485, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn add_pool_and_base_pool_liquidity	(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1733 + b * (187 ±0) + m * (117 ±0)`
		//  Estimated: `7572 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 703_438_000 picoseconds.
		Weight::from_parts(153_195_964, 7572)
			// Standard Error: 149_131
			.saturating_add(Weight::from_parts(44_949_828, 0).saturating_mul(b.into()))
			// Standard Error: 149_131
			.saturating_add(Weight::from_parts(55_200_238, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn remove_pool_and_base_pool_liquidity	(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1978 + b * (187 ±0) + m * (185 ±0)`
		//  Estimated: `7572 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 556_584_000 picoseconds.
		Weight::from_parts(166_130_458, 7572)
			// Standard Error: 92_741
			.saturating_add(Weight::from_parts(33_437_802, 0).saturating_mul(b.into()))
			// Standard Error: 92_741
			.saturating_add(Weight::from_parts(33_260_031, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_pool_and_base_pool_liquidity_one_currency	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4307`
		//  Estimated: `13940`
		// Minimum execution time: 346_995_000 picoseconds.
		Weight::from_parts(349_299_000, 13940)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:15 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_pool_from_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4537`
		//  Estimated: `39840`
		// Minimum execution time: 418_448_000 picoseconds.
		Weight::from_parts(420_742_000, 39840)
			.saturating_add(T::DbWeight::get().reads(20_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn swap_pool_to_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4337`
		//  Estimated: `16530`
		// Minimum execution time: 325_201_000 picoseconds.
		Weight::from_parts(328_067_000, 16530)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_meta_pool_underlying	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2680`
		//  Estimated: `11350`
		// Minimum execution time: 161_353_000 picoseconds.
		Weight::from_parts(162_265_000, 11350)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn update_fee_receiver	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 33_076_000 picoseconds.
		Weight::from_parts(33_587_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn set_swap_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 33_607_000 picoseconds.
		Weight::from_parts(34_058_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn set_admin_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 33_196_000 picoseconds.
		Weight::from_parts(33_427_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn ramp_a	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `968`
		//  Estimated: `4281`
		// Minimum execution time: 41_062_000 picoseconds.
		Weight::from_parts(41_213_000, 4281)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn stop_ramp_a	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `968`
		//  Estimated: `4281`
		// Minimum execution time: 40_882_000 picoseconds.
		Weight::from_parts(41_273_000, 4281)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	fn withdraw_admin_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2064`
		//  Estimated: `26890`
		// Minimum execution time: 170_291_000 picoseconds.
		Weight::from_parts(171_493_000, 26890)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}