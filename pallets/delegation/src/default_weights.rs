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

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/delegation/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for delegation.
pub trait WeightInfo {
	fn create_hierarchy() -> Weight;
	fn add_delegation() -> Weight;
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight;
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight;
	fn remove_delegation(r: u32, ) -> Weight;
	fn reclaim_deposit(r: u32, ) -> Weight;
	fn can_attest() -> Weight;
	fn can_revoke(c: u32, ) -> Weight;
	fn can_remove(c: u32, ) -> Weight;
	fn change_deposit_owner() -> Weight;
	fn update_deposit() -> Weight;
}

/// Weights for delegation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		Weight::from_ref_time(57_306_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		Weight::from_ref_time(62_800_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(32_902_000 as u64)
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(13_890_000 as u64).saturating_mul(r as u64))
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(399_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(48_305_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(4_062_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	/// The range of component `r` is `[1, 5]`.
	fn remove_delegation(r: u32, ) -> Weight {
		Weight::from_ref_time(68_555_000 as u64)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(24_147_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	/// The range of component `r` is `[1, 5]`.
	fn reclaim_deposit(r: u32, ) -> Weight {
		Weight::from_ref_time(61_377_000 as u64)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(24_028_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		Weight::from_ref_time(18_398_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	/// The range of component `c` is `[1, 5]`.
	fn can_revoke(c: u32, ) -> Weight {
		Weight::from_ref_time(13_088_000 as u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(4_399_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	/// The range of component `c` is `[1, 5]`.
	fn can_remove(c: u32, ) -> Weight {
		Weight::from_ref_time(13_085_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(4_401_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(69_043_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(64_426_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		Weight::from_ref_time(57_306_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		Weight::from_ref_time(62_800_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(32_902_000 as u64)
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(13_890_000 as u64).saturating_mul(r as u64))
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(399_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(48_305_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(4_062_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	/// The range of component `r` is `[1, 5]`.
	fn remove_delegation(r: u32, ) -> Weight {
		Weight::from_ref_time(68_555_000 as u64)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(24_147_000 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	/// The range of component `r` is `[1, 5]`.
	fn reclaim_deposit(r: u32, ) -> Weight {
		Weight::from_ref_time(61_377_000 as u64)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(24_028_000 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		Weight::from_ref_time(18_398_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	/// The range of component `c` is `[1, 5]`.
	fn can_revoke(c: u32, ) -> Weight {
		Weight::from_ref_time(13_088_000 as u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(4_399_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	/// The range of component `c` is `[1, 5]`.
	fn can_remove(c: u32, ) -> Weight {
		Weight::from_ref_time(13_085_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(4_401_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(69_043_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(64_426_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
