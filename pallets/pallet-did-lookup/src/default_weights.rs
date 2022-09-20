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

//! Autogenerated weights for pallet_did_lookup
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=pallet-did-lookup
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/pallet-did-lookup/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_did_lookup.
pub trait WeightInfo {
	fn associate_account_multisig_sr25519() -> Weight;
	fn associate_account_multisig_ed25519() -> Weight;
	fn associate_account_multisig_ecdsa() -> Weight;
	fn associate_eth_account() -> Weight;
	fn associate_sender() -> Weight;
	fn remove_sender_association() -> Weight;
	fn remove_account_association() -> Weight;
	fn transfer_deposit() -> Weight;
}

/// Weights for pallet_did_lookup using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_sr25519() -> Weight {
		Weight::from_ref_time(116_556_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ed25519() -> Weight {
		Weight::from_ref_time(114_104_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ecdsa() -> Weight {
		Weight::from_ref_time(105_595_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_eth_account() -> Weight {
		Weight::from_ref_time(110_091_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_sender() -> Weight {
		Weight::from_ref_time(58_803_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_sender_association() -> Weight {
		Weight::from_ref_time(37_730_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_account_association() -> Weight {
		Weight::from_ref_time(40_466_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn transfer_deposit() -> Weight {
		(40_466_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_sr25519() -> Weight {
		Weight::from_ref_time(116_556_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ed25519() -> Weight {
		Weight::from_ref_time(114_104_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ecdsa() -> Weight {
		Weight::from_ref_time(105_595_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_eth_account() -> Weight {
		Weight::from_ref_time(110_091_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_sender() -> Weight {
		Weight::from_ref_time(58_803_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_sender_association() -> Weight {
		Weight::from_ref_time(37_730_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_account_association() -> Weight {
		Weight::from_ref_time(40_466_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn transfer_deposit() -> Weight {
		(40_466_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
