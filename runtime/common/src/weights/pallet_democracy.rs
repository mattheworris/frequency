//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.0.0
//! DATE: 2025-04-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-0-75`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_democracy
// --extrinsic
// *
// --heap-pages=4096
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs
// --additional-trie-layers=3
// --runtime=./scripts/../target/release/wbuild/frequency-runtime/frequency_runtime.wasm
// --genesis-builder=runtime

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_democracy` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Democracy::PublicPropCount` (r:1 w:1)
	/// Proof: `Democracy::PublicPropCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:0 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4768`
		//  Estimated: `18682`
		// Minimum execution time: 49_431_000 picoseconds.
		Weight::from_parts(50_777_000, 18682)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3523`
		//  Estimated: `7190`
		// Minimum execution time: 43_350_000 picoseconds.
		Weight::from_parts(45_145_000, 7190)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3399`
		//  Estimated: `7755`
		// Minimum execution time: 63_651_000 picoseconds.
		Weight::from_parts(65_855_000, 7755)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3421`
		//  Estimated: `7755`
		// Minimum execution time: 65_877_000 picoseconds.
		Weight::from_parts(67_776_000, 7755)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Cancellations` (r:1 w:1)
	/// Proof: `Democracy::Cancellations` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `333`
		//  Estimated: `4161`
		// Minimum execution time: 26_996_000 picoseconds.
		Weight::from_parts(27_851_000, 4161)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:3 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:0 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5877`
		//  Estimated: `18682`
		// Minimum execution time: 100_860_000 picoseconds.
		Weight::from_parts(102_947_000, 18682)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3383`
		//  Estimated: `7198`
		// Minimum execution time: 13_520_000 picoseconds.
		Weight::from_parts(13_942_000, 7198)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_809_000 picoseconds.
		Weight::from_parts(2_970_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_012_000 picoseconds.
		Weight::from_parts(3_174_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:1)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:2)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `4013`
		// Minimum execution time: 24_003_000 picoseconds.
		Weight::from_parts(25_046_000, 4013)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3486`
		//  Estimated: `7198`
		// Minimum execution time: 30_188_000 picoseconds.
		Weight::from_parts(31_241_000, 7198)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5788`
		//  Estimated: `18682`
		// Minimum execution time: 82_656_000 picoseconds.
		Weight::from_parts(84_413_000, 18682)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `4013`
		// Minimum execution time: 17_500_000 picoseconds.
		Weight::from_parts(18_037_000, 4013)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (86 ±0)`
		//  Estimated: `1984 + r * (2676 ±0)`
		// Minimum execution time: 6_441_000 picoseconds.
		Weight::from_parts(4_915_113, 1984)
			// Standard Error: 10_830
			.saturating_add(Weight::from_parts(3_862_289, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::LastTabledWasExternal` (r:1 w:0)
	/// Proof: `Democracy::LastTabledWasExternal` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (86 ±0)`
		//  Estimated: `18682 + r * (2676 ±0)`
		// Minimum execution time: 9_497_000 picoseconds.
		Weight::from_parts(9_533_311, 18682)
			// Standard Error: 10_771
			.saturating_add(Weight::from_parts(3_821_356, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:3 w:3)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `763 + r * (108 ±0)`
		//  Estimated: `20295 + r * (2676 ±0)`
		// Minimum execution time: 45_058_000 picoseconds.
		Weight::from_parts(48_558_163, 20295)
			// Standard Error: 11_645
			.saturating_add(Weight::from_parts(4_824_235, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:2 w:2)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `460 + r * (108 ±0)`
		//  Estimated: `14025 + r * (2676 ±0)`
		// Minimum execution time: 22_927_000 picoseconds.
		Weight::from_parts(19_190_792, 14025)
			// Standard Error: 11_986
			.saturating_add(Weight::from_parts(4_853_299, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::PublicProps` (r:0 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_813_000 picoseconds.
		Weight::from_parts(3_023_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `7755`
		// Minimum execution time: 24_864_000 picoseconds.
		Weight::from_parts(39_086_568, 7755)
			// Standard Error: 3_334
			.saturating_add(Weight::from_parts(121_581, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493 + r * (22 ±0)`
		//  Estimated: `7755`
		// Minimum execution time: 37_016_000 picoseconds.
		Weight::from_parts(40_537_486, 7755)
			// Standard Error: 1_524
			.saturating_add(Weight::from_parts(142_693, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695 + r * (26 ±0)`
		//  Estimated: `7755`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(23_218_556, 7755)
			// Standard Error: 1_631
			.saturating_add(Weight::from_parts(139_193, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695 + r * (26 ±0)`
		//  Estimated: `7755`
		// Minimum execution time: 19_228_000 picoseconds.
		Weight::from_parts(23_701_433, 7755)
			// Standard Error: 1_875
			.saturating_add(Weight::from_parts(140_553, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `4051`
		// Minimum execution time: 18_728_000 picoseconds.
		Weight::from_parts(19_272_000, 4051)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `4013`
		// Minimum execution time: 15_535_000 picoseconds.
		Weight::from_parts(16_239_000, 4013)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4817`
		//  Estimated: `18682`
		// Minimum execution time: 48_409_000 picoseconds.
		Weight::from_parts(48_948_000, 18682)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4789`
		//  Estimated: `18682`
		// Minimum execution time: 43_589_000 picoseconds.
		Weight::from_parts(44_562_000, 18682)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `4051`
		// Minimum execution time: 14_422_000 picoseconds.
		Weight::from_parts(14_966_000, 4051)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `4161`
		// Minimum execution time: 18_737_000 picoseconds.
		Weight::from_parts(19_623_000, 4161)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use crate::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use crate::weights::extrinsic_weights::ExtrinsicBaseWeight;

  #[allow(dead_code)]
  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_propose() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_second() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7190
		);
	}
	#[test]
	fn test_vote_new() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_vote_existing() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_emergency_cancel() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4161
		);
	}
	#[test]
	fn test_blacklist() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_external_propose() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7198
		);
	}
	#[test]
	fn test_fast_track() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4013
		);
	}
	#[test]
	fn test_veto_external() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7198
		);
	}
	#[test]
	fn test_cancel_proposal() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_cancel_referendum() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4013
		);
	}
	#[test]
	fn test_on_initialize_base() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1984
		);
	}
	#[test]
	fn test_on_initialize_base_with_launch_period() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_delegate() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 20295
		);
	}
	#[test]
	fn test_undelegate() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 14025
		);
	}
	#[test]
	fn test_unlock_remove() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_unlock_set() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_remove_vote() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_remove_other_vote() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7755
		);
	}
	#[test]
	fn test_set_external_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4051
		);
	}
	#[test]
	fn test_clear_external_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4013
		);
	}
	#[test]
	fn test_set_proposal_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_clear_proposal_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 18682
		);
	}
	#[test]
	fn test_set_referendum_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4051
		);
	}
	#[test]
	fn test_clear_referendum_metadata() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4161
		);
	}
}
