// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

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

use frame_support::{
	pallet_prelude::DispatchResult,
	traits::{
		fungible::{
			freeze::{Inspect as InspectFreeze, Mutate as MutateFreeze},
			hold::{Inspect as InspectHold, Mutate as MutateHold},
		},
		LockIdentifier, LockableCurrency, ReservableCurrency,
	},
};
use sp_runtime::SaturatedConversion;

use crate::deposit::HFIdentifier;

/// Checks some precondition of the migrations.
pub fn has_user_holds<
	AccountId,
	Currency: ReservableCurrency<AccountId> + MutateHold<AccountId> + InspectHold<AccountId, Reason = HFIdentifier>,
>(
	owner: &AccountId,
	reason: &HFIdentifier,
) -> bool {
	Currency::balance_on_hold(reason, owner).saturated_into::<usize>() == 0
		&& Currency::reserved_balance(owner).saturated_into::<usize>() > 0
}

pub fn has_user_freezes<
	AccountId,
	Currency: ReservableCurrency<AccountId> + MutateFreeze<AccountId> + InspectFreeze<AccountId, Id = HFIdentifier>,
>(
	owner: &AccountId,
	reason: &HFIdentifier,
) -> bool {
	Currency::balance_frozen(reason, owner).saturated_into::<usize>() > 0
}

pub fn switch_reserved_to_hold<
	AccountId,
	Currency: ReservableCurrency<AccountId> + MutateHold<AccountId> + InspectHold<AccountId, Reason = HFIdentifier>,
>(
	owner: AccountId,
	reason: &HFIdentifier,
	amount: u128,
) -> DispatchResult {
	Currency::unreserve(&owner, amount.saturated_into());
	Currency::hold(reason, &owner, amount.saturated_into())
}

pub fn switch_locks_to_freeze<
	AccountId,
	Currency: LockableCurrency<AccountId> + MutateFreeze<AccountId, Id = HFIdentifier>,
>(
	who: AccountId,
	id_lock: LockIdentifier,
	id_freeze: &HFIdentifier,
	amount: u128,
) -> DispatchResult {
	Currency::remove_lock(id_lock, &who);
	Currency::set_freeze(id_freeze, &who, amount.saturated_into())
}
