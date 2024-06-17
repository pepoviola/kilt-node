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

#![cfg_attr(not(feature = "std"), no_std)]

mod swap;
mod traits;

use parity_scale_codec::{Decode, Encode};
use sp_runtime::traits::TrailingZeroInput;
use xcm::{VersionedAssetId, VersionedInteriorMultiLocation};

pub use crate::pallet::*;

const LOG_TARGET: &str = "runtime::pallet-asset-swap";

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		swap::{SwapPairInfo, SwapPairRatio, SwapRequestLocalAsset},
		traits::SwapHooks,
		LOG_TARGET,
	};

	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::Inspect as InspectFungible,
			fungibles::{Inspect as InspectFungibles, Mutate as MutateFungibles},
			EnsureOrigin,
		},
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::*;
	use xcm::{VersionedAssetId, VersionedInteriorMultiLocation, VersionedMultiLocation};

	pub type LocalAssetsBalanceOf<T> =
		<<T as Config>::LocalAssets as InspectFungibles<<T as frame_system::Config>::AccountId>>::Balance;
	pub type SwapPairInfoOf<T> = SwapPairInfo<<T as frame_system::Config>::AccountId>;
	pub type SwapRequestLocalAssetOf<T> = SwapRequestLocalAsset<LocalAssetsBalanceOf<T>>;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: InspectFungible<Self::AccountId>;
		type LocalAssets: MutateFungibles<Self::AccountId>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type PauseOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type SwapHooks: SwapHooks;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SwapPairCreated {
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
			ratio: SwapPairRatio,
			maximum_issuance: u128,
			pool_account: T::AccountId,
		},
		SwapPairRemoved {
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		LocalAssetExisting,
		LocalAssetNotFound,
		AssetIdMismatch,
		Hook(u8),
		Internal,
	}

	#[pallet::storage]
	#[pallet::getter(fn remote_asset_for_local_asset)]
	pub(crate) type LocalToRemoteAssets<T> =
		StorageMap<_, Blake2_128Concat, VersionedInteriorMultiLocation, SwapPairInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn local_asset_for_remote_asset)]
	pub(crate) type RemoteToLocalAssets<T> =
		StorageMap<_, Blake2_128Concat, VersionedAssetId, (VersionedMultiLocation, VersionedInteriorMultiLocation)>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(u64::MAX)]
		pub fn create_swap_pair(
			origin: OriginFor<T>,
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			reserve_location_base: Box<VersionedMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
			ratio: SwapPairRatio,
			maximum_issuance: u128,
		) -> DispatchResult {
			T::ManagerOrigin::ensure_origin(origin)?;

			if LocalToRemoteAssets::<T>::contains_key(&(*local_asset_id)) {
				return Err(Error::<T>::LocalAssetExisting.into());
			};
			if RemoteToLocalAssets::<T>::contains_key(&(*remote_asset_id)) {
				log::error!(target: LOG_TARGET, "Found an entry in `RemoteToLocalAssets` for remote asset ID: {:?} when there should not be any.", remote_asset_id);
				return Err(Error::<T>::Internal.into());
			};

			let pool_account = Self::pool_account_id_for_swap_pair(&local_asset_id, &remote_asset_id)?;
			let swap_pair_info = SwapPairInfoOf::<T> {
				pool_account: pool_account.clone(),
				ratio: ratio.clone(),
				remote_asset_balance: maximum_issuance,
				remote_asset_id: *remote_asset_id.clone(),
				running: false,
			};
			LocalToRemoteAssets::<T>::insert(&(*local_asset_id), swap_pair_info);
			RemoteToLocalAssets::<T>::insert(&(*remote_asset_id), (*reserve_location_base, *local_asset_id.clone()));

			T::SwapHooks::on_swap_pair_created(&local_asset_id, &remote_asset_id)
				.map_err(|e| Error::<T>::Hook(e.into()))?;

			Self::deposit_event(Event::<T>::SwapPairCreated {
				local_asset_id,
				maximum_issuance,
				pool_account,
				ratio,
				remote_asset_id,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(u64::MAX)]
		pub fn remove_swap_pair(
			origin: OriginFor<T>,
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
		) -> DispatchResult {
			T::ManagerOrigin::ensure_origin(origin)?;

			let swap_pair = LocalToRemoteAssets::<T>::take(&(*local_asset_id)).ok_or(Error::<T>::LocalAssetNotFound)?;
			ensure!(
				swap_pair.remote_asset_id == *remote_asset_id,
				Error::<T>::AssetIdMismatch
			);
			if RemoteToLocalAssets::<T>::take(&(*remote_asset_id)).is_none() {
				log::error!(target: LOG_TARGET, "Entry in `RemoteToLocalAssets` for remote asset ID: {:?} not found when there should have been one.", remote_asset_id);
				return Err(Error::<T>::Internal.into());
			}

			T::SwapHooks::on_swap_pair_removed(&local_asset_id, &remote_asset_id)
				.map_err(|e| Error::<T>::Hook(e.into()))?;

			Self::deposit_event(Event::<T>::SwapPairRemoved {
				local_asset_id,
				remote_asset_id,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(u64::MAX)]
		pub fn pause_swap_pair(
			origin: OriginFor<T>,
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
		) -> DispatchResult {
			T::PauseOrigin::ensure_origin(origin)?;

			LocalToRemoteAssets::<T>::try_mutate(&(*local_asset_id), |entry| {
				let existing_entry = entry.as_mut().ok_or(Error::<T>::LocalAssetNotFound)?;
				ensure!(
					existing_entry.remote_asset_id == *remote_asset_id,
					Error::<T>::AssetIdMismatch
				);
				existing_entry.running = false;
				Ok::<_, Error<T>>(())
			})?;

			T::SwapHooks::on_swap_pair_paused(&local_asset_id, &remote_asset_id)
				.map_err(|e| Error::<T>::Hook(e.into()))?;

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(u64::MAX)]
		pub fn resume_swap_pair(
			origin: OriginFor<T>,
			local_asset_id: Box<VersionedInteriorMultiLocation>,
			remote_asset_id: Box<VersionedAssetId>,
		) -> DispatchResult {
			T::ManagerOrigin::ensure_origin(origin)?;

			LocalToRemoteAssets::<T>::try_mutate(&(*local_asset_id), |entry| {
				let existing_entry = entry.as_mut().ok_or(Error::<T>::LocalAssetNotFound)?;
				ensure!(
					existing_entry.remote_asset_id == *remote_asset_id,
					Error::<T>::AssetIdMismatch
				);
				existing_entry.running = true;
				Ok::<_, Error<T>>(())
			})?;

			T::SwapHooks::on_swap_pair_resumed(&local_asset_id, &remote_asset_id)
				.map_err(|e| Error::<T>::Hook(e.into()))?;

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(u64::MAX)]
		pub fn swap(
			origin: OriginFor<T>,
			local_asset: Box<SwapRequestLocalAssetOf<T>>,
			remote_asset_id: Box<VersionedAssetId>,
		) -> DispatchResult {
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn pool_account_id_for_swap_pair(
		local_asset_id: &VersionedInteriorMultiLocation,
		remote_asset_id: &VersionedAssetId,
	) -> Result<T::AccountId, Error<T>> {
		// Taken and adapted from https://github.com/paritytech/polkadot-sdk/blob/796890979e5d7d16a522c304376d78eec120f3cb/substrate/frame/asset-conversion/src/types.rs#L161.
		let hash_input = (local_asset_id, b'.', remote_asset_id).encode();
		let hash_output = sp_io::hashing::blake2_256(hash_input.as_slice());
		T::AccountId::decode(&mut TrailingZeroInput::new(hash_output.as_slice())).map_err(|e| {
			log::error!(target: LOG_TARGET, "Failed to generate pool ID from given pair ({:?}, {:?}) with error: {:?}", local_asset_id, remote_asset_id, e);
			Error::<T>::Internal
		})
	}
}
