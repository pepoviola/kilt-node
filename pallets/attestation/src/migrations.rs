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
	traits::{Get, OnRuntimeUpgrade, ReservableCurrency},
	weights::Weight,
};
use kilt_support::{
	deposit::{HFIdentifier, Pallets},
	migration::switch_reserved_to_hold,
};
use log;
use sp_runtime::SaturatedConversion;
use sp_std::marker::PhantomData;

use crate::{AccountIdOf, AttestationDetails, Attestations, Config, CurrencyOf};

pub struct BalanceMigration<T>(PhantomData<T>);

impl<T: crate::pallet::Config> OnRuntimeUpgrade for BalanceMigration<T>
where
	<T as Config>::Currency: ReservableCurrency<T::AccountId>,
{
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		log::info!("Attestation: Initiating migration");
		if is_upgraded::<T>() {
			return do_migration::<T>();
		}

		log::info!("Attestation: No migration needed. This file should be deleted.");
		<T as frame_system::Config>::DbWeight::get().reads_writes(0, 0)
	}

	// 	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<sp_std::vec::Vec<u8>, &'static str> {
		use frame_support::ensure;
		use sp_std::vec;
		// before the upgrade, there should be no account with holds
		ensure!(is_upgraded::<T>(), "Pre upgrade: there are users with holds.");
		log::info!("Attestation: There are no users with holds!");

		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_pre_state: sp_std::vec::Vec<u8>) -> Result<(), &'static str> {
		use frame_support::ensure;

		// before the upgrade, there should be no account with holds
		ensure!(!is_upgraded::<T>(), "Post upgrade: there are users with reserves.");
		log::info!("Attestation: Post migration checks");

		Ok(())
	}
}

/// Checks if there is an user, who has still reserved balance and no holds. If
/// yes, the migration is not executed yet.
fn is_upgraded<T: Config>() -> bool
where
	<T as Config>::Currency: ReservableCurrency<T::AccountId>,
{
	Attestations::<T>::iter_values()
		.map(|details: AttestationDetails<T>| true)
		.any(|user| !user)
}

fn do_migration<T: Config>() -> Weight
where
	<T as Config>::Currency: ReservableCurrency<T::AccountId>,
{
	Attestations::<T>::iter()
		.map(|(key, attestations_detail)| -> Weight {
			let deposit = attestations_detail.deposit;
			let error = switch_reserved_to_hold::<AccountIdOf<T>, CurrencyOf<T>>(
				deposit.owner,
				&HFIdentifier::Deposit(Pallets::Attestation),
				deposit.amount.saturated_into(),
			);

			if error.is_ok() {
				return <T as frame_system::Config>::DbWeight::get().reads_writes(1, 1);
			}

			log::error!("{:?}", error);

			log::error!(
				" Attestation: Could not convert reserves to hold from attestation: {:?} ",
				key
			);

			<T as frame_system::Config>::DbWeight::get().reads_writes(0, 0)
		})
		.fold(Weight::zero(), |acc, next| acc.saturating_add(next))
}
