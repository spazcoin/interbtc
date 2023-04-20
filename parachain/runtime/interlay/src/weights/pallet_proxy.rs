
//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
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
// 100
// --repeat
// 10
// --output
// /tmp/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_proxy using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `231 + p * (37 ±0)`
		//  Estimated: `3716`
		// Minimum execution time: 17_236_000 picoseconds.
		Weight::from_parts(19_070_354, 3716)
			// Standard Error: 7_938
			.saturating_add(Weight::from_parts(40_181, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `834 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `11014`
		// Minimum execution time: 35_907_000 picoseconds.
		Weight::from_parts(35_425_222, 11014)
			// Standard Error: 11_345
			.saturating_add(Weight::from_parts(210_460, 0).saturating_mul(a.into()))
			// Standard Error: 11_733
			.saturating_add(Weight::from_parts(103_673, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `717 + a * (68 ±0)`
		//  Estimated: `7298`
		// Minimum execution time: 23_835_000 picoseconds.
		Weight::from_parts(29_849_221, 7298)
			// Standard Error: 14_162
			.saturating_add(Weight::from_parts(24_872, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `717 + a * (68 ±0)`
		//  Estimated: `7298`
		// Minimum execution time: 22_647_000 picoseconds.
		Weight::from_parts(22_200_202, 7298)
			// Standard Error: 12_839
			.saturating_add(Weight::from_parts(264_287, 0).saturating_mul(a.into()))
			// Standard Error: 13_277
			.saturating_add(Weight::from_parts(67_036, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `766 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `11014`
		// Minimum execution time: 35_124_000 picoseconds.
		Weight::from_parts(38_800_299, 11014)
			// Standard Error: 23_749
			.saturating_add(Weight::from_parts(238_701, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548 + p * (37 ±0)`
		//  Estimated: `6306`
		// Minimum execution time: 30_929_000 picoseconds.
		Weight::from_parts(35_589_594, 6306)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548 + p * (37 ±0)`
		//  Estimated: `6306`
		// Minimum execution time: 27_482_000 picoseconds.
		Weight::from_parts(29_303_058, 6306)
			// Standard Error: 3_507
			.saturating_add(Weight::from_parts(43_111, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548 + p * (37 ±0)`
		//  Estimated: `6306`
		// Minimum execution time: 21_592_000 picoseconds.
		Weight::from_parts(23_958_854, 6306)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `528`
		//  Estimated: `6306`
		// Minimum execution time: 32_049_000 picoseconds.
		Weight::from_parts(34_661_593, 6306)
			// Standard Error: 5_483
			.saturating_add(Weight::from_parts(40_050, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `619 + p * (37 ±0)`
		//  Estimated: `6306`
		// Minimum execution time: 22_887_000 picoseconds.
		Weight::from_parts(23_779_375, 6306)
			// Standard Error: 15_534
			.saturating_add(Weight::from_parts(154_543, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}