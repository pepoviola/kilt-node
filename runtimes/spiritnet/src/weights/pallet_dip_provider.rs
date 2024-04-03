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

//! Autogenerated weights for `pallet_dip_provider`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/runtime-weight-template.hbs
// --header=HEADER-GPL
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=spiritnet-dev
// --pallet=pallet-dip-provider
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/pallet_dip_provider.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_dip_provider`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dip_provider::WeightInfo for WeightInfo<T> {
	/// Storage: `Did::DidBlacklist` (r:1 w:0)
	/// Proof: `Did::DidBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Did::Did` (r:1 w:0)
	/// Proof: `Did::Did` (`max_values`: None, `max_size`: Some(2312), added: 4787, mode: `MaxEncodedLen`)
	/// Storage: `Web3Names::Names` (r:1 w:0)
	/// Proof: `Web3Names::Names` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Web3Names::Owner` (r:1 w:0)
	/// Proof: `Web3Names::Owner` (`max_values`: None, `max_size`: Some(137), added: 2612, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:11 w:0)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `DipProvider::IdentityCommitments` (r:1 w:1)
	/// Proof: `DipProvider::IdentityCommitments` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `DepositStorage::Deposits` (r:1 w:1)
	/// Proof: `DepositStorage::Deposits` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn commit_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3425`
		//  Estimated: `29282`
		// Minimum execution time: 217_916_000 picoseconds.
		Weight::from_parts(222_584_000, 0)
			.saturating_add(Weight::from_parts(0, 29282))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `DipProvider::IdentityCommitments` (r:1 w:1)
	/// Proof: `DipProvider::IdentityCommitments` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `DepositStorage::Deposits` (r:1 w:1)
	/// Proof: `DepositStorage::Deposits` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn delete_identity_commitment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1107`
		//  Estimated: `4414`
		// Minimum execution time: 61_645_000 picoseconds.
		Weight::from_parts(62_180_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_commit_identity() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 29282
		);
	}
	#[test]
	fn test_delete_identity_commitment() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
}
