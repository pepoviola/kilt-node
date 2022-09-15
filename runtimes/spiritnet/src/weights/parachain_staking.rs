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

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-23, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=1
// --repeat=20
// --pallet=parachain-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/parachain_staking.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking Round (r:1 w:0)
	fn on_initialize_no_action() -> Weight {
		(3_103_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		(11_496_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		(35_227_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		(5_101_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking RewardCount (r:72 w:72)
	// Storage: ParachainStaking Rewards (r:2 w:2)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	/// The range of component `n` is `[0, 75]`.
	/// The range of component `m` is `[0, 35]`.
	fn set_inflation(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 3_005_000
			.saturating_add((216_364_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 6_440_000
			.saturating_add((440_763_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads((37 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((75 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((36 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((75 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:59 w:0)
	/// The range of component `n` is `[16, 75]`.
	/// The range of component `m` is `[0, 35]`.
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 524_000
			.saturating_add((5_444_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 883_000
			.saturating_add((5_252_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		(24_978_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: ParachainStaking RewardCount (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	/// The range of component `n` is `[17, 75]`.
	/// The range of component `m` is `[0, 35]`.
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_056_000
			.saturating_add((2_682_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_750_000
			.saturating_add((22_787_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	/// The range of component `n` is `[1, 74]`.
	/// The range of component `m` is `[0, 35]`.
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		(31_764_000 as Weight)
			// Standard Error: 644_000
			.saturating_add((1_293_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_343_000
			.saturating_add((2_377_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	/// The range of component `n` is `[17, 74]`.
	/// The range of component `m` is `[0, 35]`.
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 987_000
			.saturating_add((7_127_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_607_000
			.saturating_add((6_771_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	/// The range of component `n` is `[17, 74]`.
	/// The range of component `m` is `[0, 35]`.
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 26_390_000
			.saturating_add((13_197_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 42_978_000
			.saturating_add((24_662_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: ParachainStaking RewardCount (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	/// The range of component `n` is `[17, 74]`.
	/// The range of component `m` is `[0, 35]`.
	fn execute_leave_candidates(_n: u32, m: u32, ) -> Weight {
		(998_775_000 as Weight)
			// Standard Error: 431_000
			.saturating_add((20_295_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 74]`.
	/// The range of component `m` is `[0, 35]`.
	/// The range of component `u` is `[0, 9]`.
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 332_000
			.saturating_add((2_506_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 693_000
			.saturating_add((9_543_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_698_000
			.saturating_add((5_104_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `n` is `[1, 74]`.
	/// The range of component `m` is `[0, 35]`.
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 905_000
			.saturating_add((2_785_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_888_000
			.saturating_add((10_300_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking LastDelegation (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:1 w:1)
	/// The range of component `n` is `[1, 75]`.
	/// The range of component `m` is `[1, 34]`.
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		(8_951_000 as Weight)
			// Standard Error: 855_000
			.saturating_add((1_589_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_919_000
			.saturating_add((3_562_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:2 w:0)
	/// The range of component `n` is `[1, 75]`.
	/// The range of component `m` is `[1, 34]`.
	/// The range of component `u` is `[1, 9]`.
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(56_774_000 as Weight)
			// Standard Error: 883_000
			.saturating_add((1_562_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_981_000
			.saturating_add((867_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 8_175_000
			.saturating_add((5_717_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:2 w:0)
	/// The range of component `n` is `[1, 75]`.
	/// The range of component `m` is `[1, 34]`.
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		(1_844_000 as Weight)
			// Standard Error: 138_000
			.saturating_add((1_192_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 310_000
			.saturating_add((2_218_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking RewardCount (r:2 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	/// The range of component `n` is `[1, 75]`.
	/// The range of component `m` is `[1, 34]`.
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		(3_824_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((1_216_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 114_000
			.saturating_add((2_150_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `u` is `[1, 9]`.
	fn unlock_unstaked(_u: u32, ) -> Weight {
		(30_399_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		(13_991_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking RewardCount (r:2 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn increment_delegator_rewards() -> Weight {
		(25_796_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking RewardCount (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// The range of component `m` is `[0, 35]`.
	fn increment_collator_rewards(_m: u32, ) -> Weight {
		(366_611_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(75 as Weight))
			.saturating_add(T::DbWeight::get().writes(72 as Weight))
	}
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_rewards() -> Weight {
		(29_833_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking RewardCount (r:72 w:72)
	// Storage: ParachainStaking Rewards (r:2 w:2)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:0)
	/// The range of component `n` is `[0, 75]`.
	/// The range of component `m` is `[0, 35]`.
	fn execute_scheduled_reward_change(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 5_730_000
			.saturating_add((202_623_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 12_280_000
			.saturating_add((415_436_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads((37 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((75 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((36 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((75 as Weight).saturating_mul(m as Weight)))
	}
}
