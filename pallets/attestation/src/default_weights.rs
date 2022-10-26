// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for attestation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-26, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=1
// --pallet=attestation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/attestation/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for attestation.
pub trait WeightInfo {
	fn add() -> Weight;
	fn revoke() -> Weight;
	fn remove() -> Weight;
	fn reclaim_deposit() -> Weight;
	fn change_deposit_owner() -> Weight;
	fn update_deposit() -> Weight;
}

/// Weights for attestation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add() -> Weight {
		Weight::from_ref_time(75_976_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	fn revoke() -> Weight {
		Weight::from_ref_time(42_119_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove() -> Weight {
		Weight::from_ref_time(60_200_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reclaim_deposit() -> Weight {
		Weight::from_ref_time(59_527_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(79_907_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(71_729_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add() -> Weight {
		Weight::from_ref_time(75_976_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	fn revoke() -> Weight {
		Weight::from_ref_time(42_119_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove() -> Weight {
		Weight::from_ref_time(60_200_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reclaim_deposit() -> Weight {
		Weight::from_ref_time(59_527_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(79_907_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Attestation Attestations (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(71_729_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}