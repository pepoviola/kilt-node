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

// TODO: Pallet description

#![cfg_attr(not(feature = "std"), no_std)]

mod origin;
pub mod traits;

pub use crate::{origin::*, pallet::*};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::Dispatchable, pallet_prelude::*, traits::EnsureOrigin, Twox64Concat};
	use frame_system::pallet_prelude::*;

	use dip_support::{latest::IdentityProofAction, VersionedIdentityProof, VersionedIdentityProofAction};

	use crate::traits::IdentityProofVerifier;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	// TODO: Store also additional details received by the provider
	// TODO: Make pub(crate) again
	#[pallet::storage]
	#[pallet::getter(fn identity_proofs)]
	pub type IdentityProofs<T> = StorageMap<_, Twox64Concat, <T as Config>::Identifier, <T as Config>::ProofDigest>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Identifier: Parameter + MaxEncodedLen;
		type ProofLeafKey: Parameter;
		type ProofLeafValue: Parameter;
		type ProofDigest: Parameter + MaxEncodedLen;
		type ProofVerifier: IdentityProofVerifier<
			ProofDigest = Self::ProofDigest,
			LeafKey = Self::ProofLeafKey,
			LeafValue = Self::ProofLeafValue,
		>;
		type RuntimeCall: Parameter + Dispatchable<RuntimeOrigin = <Self as Config>::RuntimeOrigin>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type RuntimeOrigin: From<Origin<Self>>
			+ From<<Self as frame_system::Config>::RuntimeOrigin>
			+ Into<Result<cumulus_pallet_xcm::Origin, <Self as Config>::RuntimeOrigin>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IdentityInfoUpdated(T::Identifier, T::ProofDigest),
		IdentityInfoDeleted(T::Identifier),
		// TODO: Remove
		Error1,
		Error2,
		Error3,
		Error4,
		Error5,
	}

	#[pallet::error]
	pub enum Error<T> {
		IdentityNotFound,
		InvalidProof,
		DispatchError,
	}

	#[pallet::origin]
	pub type Origin<T> = DipOrigin<<T as Config>::Identifier, <T as frame_system::Config>::AccountId>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn process_identity_action(
			origin: OriginFor<T>,
			action: VersionedIdentityProofAction<T::Identifier, T::ProofDigest>,
		) -> DispatchResult {
			#[cfg(feature = "std")]
			// println!("BBBBB 1 - origin = {:?}", origin);
			Self::deposit_event(Event::<T>::Error1);
			#[cfg(feature = "std")]
			println!("BBBBB 2");
			cumulus_pallet_xcm::ensure_sibling_para(<T as Config>::RuntimeOrigin::from(origin)).map_err(|e| {
				#[cfg(feature = "std")]
				println!("BBBBB 3 - {:?}", e);
				Self::deposit_event(Event::<T>::Error2);
				e
			})?;

			let event = match action {
				VersionedIdentityProofAction::V1(IdentityProofAction::Updated(identifier, proof, _)) => {
					#[cfg(feature = "std")]
					println!("BBBBB 4");
					Self::deposit_event(Event::<T>::Error3);
					IdentityProofs::<T>::mutate(&identifier, |entry| *entry = Some(proof.clone()));
					Event::<T>::IdentityInfoUpdated(identifier, proof)
				}
				VersionedIdentityProofAction::V1(IdentityProofAction::Deleted(identifier)) => {
					#[cfg(feature = "std")]
					println!("BBBBB 5");
					Self::deposit_event(Event::<T>::Error4);
					IdentityProofs::<T>::remove(&identifier);
					Event::<T>::IdentityInfoDeleted(identifier)
				}
			};
			#[cfg(feature = "std")]
			println!("BBBBB 6");
			Self::deposit_event(Event::<T>::Error5);

			Self::deposit_event(event);
			#[cfg(feature = "std")]
			println!("BBBBB 7");
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn dispatch_as(
			origin: OriginFor<T>,
			identifier: T::Identifier,
			proof: VersionedIdentityProof<T::ProofLeafKey, T::ProofLeafValue>,
			call: sp_std::boxed::Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			let submitter = ensure_signed(origin)?;
			let proof_digest = IdentityProofs::<T>::get(&identifier).ok_or(Error::<T>::IdentityNotFound)?;
			let _ = T::ProofVerifier::verify_proof_against_digest(proof, proof_digest)
				.map_err(|_| Error::<T>::InvalidProof)?;
			// TODO: Proper DID signature verification (and cross-chain replay protection)
			let did_origin = DipOrigin {
				did_identifier: identifier,
				account_address: submitter,
			};
			let _ = call
				.dispatch(did_origin.into())
				.map_err(|_| Error::<T>::DispatchError)?;
			Ok(())
		}
	}
}
