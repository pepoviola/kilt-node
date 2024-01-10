// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

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
//! DATE: 2023-07-25
//! STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/weight-template.hbs
// --header=HEADER-GPL
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=dev
// --pallet=attestation
// --extrinsic=*
// --output=./pallets/attestation/src/default_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

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
	/// Storage: Ctype Ctypes (r:1 w:0)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn add() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `761`
		//  Estimated: `10810`
		// Minimum execution time: 58_839 nanoseconds.
		Weight::from_parts(59_802_000, 10810)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3660`
		// Minimum execution time: 29_744 nanoseconds.
		Weight::from_parts(30_743_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 37_032 nanoseconds.
		Weight::from_parts(37_649_000, 7257)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn reclaim_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 44_231 nanoseconds.
		Weight::from_parts(73_103_000, 7257)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1389`
		//  Estimated: `9864`
		// Minimum execution time: 61_787 nanoseconds.
		Weight::from_parts(62_920_000, 9864)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 55_347 nanoseconds.
		Weight::from_parts(56_121_000, 7257)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Ctype Ctypes (r:1 w:0)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn add() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `761`
		//  Estimated: `10810`
		// Minimum execution time: 58_839 nanoseconds.
		Weight::from_parts(59_802_000, 10810)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3660`
		// Minimum execution time: 29_744 nanoseconds.
		Weight::from_parts(30_743_000, 3660)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 37_032 nanoseconds.
		Weight::from_parts(37_649_000, 7257)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn reclaim_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 44_231 nanoseconds.
		Weight::from_parts(73_103_000, 7257)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1389`
		//  Estimated: `9864`
		// Minimum execution time: 61_787 nanoseconds.
		Weight::from_parts(62_920_000, 9864)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 55_347 nanoseconds.
		Weight::from_parts(56_121_000, 7257)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
