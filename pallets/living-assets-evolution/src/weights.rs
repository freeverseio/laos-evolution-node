
//! Autogenerated weights for `pallet_living_assets_evolution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos-evolution
// benchmark
// pallet
// --chain=dev
// --pallet=pallet_living_assets_evolution
// --extrinsic=*
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output=./pallets/living-assets-evolution/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_living_assets_evolution`.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn mint_with_external_uri() -> Weight;
}

/// Weights for `pallet_living_assets_evolution` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `TemplateModule::CollectionCounter` (r:1 w:1)
	/// Proof: `TemplateModule::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::CollectionOwner` (r:0 w:1)
	/// Proof: `TemplateModule::CollectionOwner` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1493`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 1493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `TemplateModule::CollectionOwner` (r:1 w:0)
	/// Proof: `TemplateModule::CollectionOwner` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::ExplicitTokenURI` (r:0 w:1)
	/// Proof: `TemplateModule::ExplicitTokenURI` (`max_values`: None, `max_size`: Some(562), added: 3037, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::AssetOwner` (r:0 w:1)
	/// Proof: `TemplateModule::AssetOwner` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn mint_with_external_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3521`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3521)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `TemplateModule::CollectionCounter` (r:1 w:1)
	/// Proof: `TemplateModule::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::CollectionOwner` (r:0 w:1)
	/// Proof: `TemplateModule::CollectionOwner` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1493`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 1493)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `TemplateModule::CollectionOwner` (r:1 w:0)
	/// Proof: `TemplateModule::CollectionOwner` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::ExplicitTokenURI` (r:0 w:1)
	/// Proof: `TemplateModule::ExplicitTokenURI` (`max_values`: None, `max_size`: Some(562), added: 3037, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::AssetOwner` (r:0 w:1)
	/// Proof: `TemplateModule::AssetOwner` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn mint_with_external_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3521`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3521)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
