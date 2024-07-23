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

use sp_runtime::AccountId32;

use crate::mock::Balances;

mod force_set_switch_pair;
mod force_unset_switch_pair;
mod pause_switch_pair;
mod resume_switch_pair;
mod set_switch_pair;
mod switch;
mod update_remote_fee;

fn assert_supply_invariant(
	total_supply: impl Into<u128>,
	circulating_supply: impl Copy + Into<u128>,
	remote_balance: impl Into<u128>,
	pool_address: &AccountId32,
) {
	// The pool must ALWAYS have enough local tokens to exchange for remote tokens
	assert!(circulating_supply.into() <= Balances::usable_balance(pool_address) as u128);
	// The amount of calculated remote tokens controlled by the chain must always match the remote balance (including its ED).
	assert_eq!(total_supply.into() - circulating_supply.into(), remote_balance.into());
}
