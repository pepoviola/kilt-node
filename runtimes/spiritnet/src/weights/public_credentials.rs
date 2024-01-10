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

//! Autogenerated weights for `public_credentials`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=public-credentials
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/public_credentials.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `public_credentials`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> public_credentials::WeightInfo for WeightInfo<T> {
	/// Storage: `Ctype::Ctypes` (r:1 w:0)
	/// Proof: `Ctype::Ctypes` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::CredentialSubjects` (r:0 w:1)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 100000]`.
	fn add(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `797`
		//  Estimated: `4414`
		// Minimum execution time: 71_519_000 picoseconds.
		Weight::from_parts(67_427_092, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(2_453, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:0)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	fn revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852`
		//  Estimated: `3940`
		// Minimum execution time: 27_713_000 picoseconds.
		Weight::from_parts(27_954_000, 0)
			.saturating_add(Weight::from_parts(0, 3940))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:0)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	fn unrevoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852`
		//  Estimated: `3940`
		// Minimum execution time: 27_437_000 picoseconds.
		Weight::from_parts(27_806_000, 0)
			.saturating_add(Weight::from_parts(0, 3940))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:1)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1632`
		//  Estimated: `4414`
		// Minimum execution time: 70_060_000 picoseconds.
		Weight::from_parts(70_567_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:1)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn reclaim_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1632`
		//  Estimated: `4414`
		// Minimum execution time: 69_712_000 picoseconds.
		Weight::from_parts(70_357_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:0)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1473`
		//  Estimated: `7838`
		// Minimum execution time: 108_420_000 picoseconds.
		Weight::from_parts(109_554_000, 0)
			.saturating_add(Weight::from_parts(0, 7838))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `PublicCredentials::CredentialSubjects` (r:1 w:0)
	/// Proof: `PublicCredentials::CredentialSubjects` (`max_values`: None, `max_size`: Some(312), added: 2787, mode: `MaxEncodedLen`)
	/// Storage: `PublicCredentials::Credentials` (r:1 w:1)
	/// Proof: `PublicCredentials::Credentials` (`max_values`: None, `max_size`: Some(475), added: 2950, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1018`
		//  Estimated: `4414`
		// Minimum execution time: 90_675_000 picoseconds.
		Weight::from_parts(91_507_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_add() {
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
	#[test]
	fn test_revoke() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3940
		);
	}
	#[test]
	fn test_unrevoke() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3940
		);
	}
	#[test]
	fn test_remove() {
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
	#[test]
	fn test_reclaim_deposit() {
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
	#[test]
	fn test_change_deposit_owner() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7838
		);
	}
	#[test]
	fn test_update_deposit() {
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
