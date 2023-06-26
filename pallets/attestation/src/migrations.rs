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
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade, ReservableCurrency, StorageVersion},
	weights::Weight,
};
use kilt_support::migration::switch_reserved_to_hold;
use log;
use sp_runtime::SaturatedConversion;
use sp_std::marker::PhantomData;

use crate::{AccountIdOf, Attestations, Config, CurrencyOf, HoldReason, Pallet};

const CURRENT_STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
const TARGET_STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

pub struct BalanceMigration<T>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for BalanceMigration<T>
where
	<T as Config>::Currency: ReservableCurrency<T::AccountId>,
{
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		log::info!("Attestation: Initiating migration");

		let onchain_storage_version = Pallet::<T>::on_chain_storage_version();

		if onchain_storage_version.eq(&CURRENT_STORAGE_VERSION) {
			TARGET_STORAGE_VERSION.put::<Pallet<T>>();

			<T as frame_system::Config>::DbWeight::get()
				.reads_writes(1, 1)
				.saturating_add(do_migration::<T>())
		} else {
			log::info!(
			"Attestation: No migration needed. This file should be deleted. Current storage version: {:?}, Required Version for update: {:?}", 
			onchain_storage_version,
			CURRENT_STORAGE_VERSION
		);

			<T as frame_system::Config>::DbWeight::get().reads_writes(1, 0)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<sp_std::vec::Vec<u8>, &'static str> {
		use sp_std::vec;

		let has_all_user_no_holds = Attestations::<T>::iter_values()
			.map(|details: crate::AttestationDetails<T>| {
				kilt_support::migration::has_user_reserved_balance::<AccountIdOf<T>, CurrencyOf<T>>(
					&details.deposit.owner,
					&HoldReason::Deposit.into(),
				)
			})
			.all(|user| user);

		assert!(
			has_all_user_no_holds,
			"Pre Upgrade Attestation: there are users with holds!"
		);

		assert_eq!(Pallet::<T>::on_chain_storage_version(), CURRENT_STORAGE_VERSION);

		log::info!("Attestation: Pre migration checks successful");

		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_pre_state: sp_std::vec::Vec<u8>) -> Result<(), &'static str> {
		use frame_support::traits::fungible::InspectHold;

		Attestations::<T>::iter().try_for_each(|(key, details)| -> Result<(), &'static str> {
			let hold_balance: u128 =
				<T as Config>::Currency::balance_on_hold(&HoldReason::Deposit.into(), &details.deposit.owner)
					.saturated_into();
			assert!(
				details.deposit.amount.saturated_into::<u128>() <= hold_balance,
				"Attestation: Hold balance is not matching for attestation {:?}. Expected hold: {:?}. Real hold: {:?}",
				key,
				details.deposit.amount,
				hold_balance
			);

			Ok(())
		})?;

		assert_eq!(Pallet::<T>::on_chain_storage_version(), TARGET_STORAGE_VERSION);

		log::info!("Attestation: Post migration checks successful");
		Ok(())
	}
}

fn do_migration<T: Config>() -> Weight
where
	<T as Config>::Currency: ReservableCurrency<T::AccountId>,
{
	Attestations::<T>::iter()
		.map(|(key, attestations_detail)| -> Weight {
			let deposit = attestations_detail.deposit;
			let result = switch_reserved_to_hold::<AccountIdOf<T>, CurrencyOf<T>>(
				deposit.owner,
				&HoldReason::Deposit.into(),
				deposit.amount.saturated_into(),
			);

			if result.is_err() {
				log::error!(
					" Attestation: Could not convert reserves to hold from attestation: {:?} error: {:?}",
					key,
					result
				);
			}

			// Currency::reserve and Currency::hold each read and write to the DB once.
			// Since we are uncertain about which operation may fail, in the event of an
			// error, we assume the worst-case scenario here.
			<T as frame_system::Config>::DbWeight::get().reads_writes(2, 2)
		})
		.fold(Weight::zero(), |acc, next| acc.saturating_add(next))
}
