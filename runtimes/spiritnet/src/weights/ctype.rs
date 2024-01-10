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

//! Autogenerated weights for `ctype`
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
// --pallet=ctype
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/ctype.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `ctype`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ctype::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Ctype::Ctypes` (r:1 w:1)
	/// Proof: `Ctype::Ctypes` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 5242880]`.
	fn add(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `552`
		//  Estimated: `6204`
		// Minimum execution time: 53_888_000 picoseconds.
		Weight::from_parts(1_731_020, 0)
			.saturating_add(Weight::from_parts(0, 6204))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_628, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Ctype::Ctypes` (r:1 w:1)
	/// Proof: `Ctype::Ctypes` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	fn set_block_number() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120`
		//  Estimated: `3553`
		// Minimum execution time: 18_402_000 picoseconds.
		Weight::from_parts(19_211_000, 0)
			.saturating_add(Weight::from_parts(0, 3553))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
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
				> 6204
		);
	}
	#[test]
	fn test_set_block_number() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3553
		);
	}
}
