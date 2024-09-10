#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::types::{Curve, PoolDetails, TokenMeta};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::{ValueQuery, *},
		traits::{
			fungible::{Inspect as InspectFungible, Mutate, MutateHold},
			fungibles::{
				metadata::Mutate as FungiblesMetadata, Create as CreateFungibles, Destroy as DestroyFungibles,
			},
			tokens::{AssetId, Balance},
		},
	};
	use frame_system::pallet_prelude::*;
	use parity_scale_codec::EncodeLike;
	use sp_runtime::{
		traits::{EnsureAddAssign, One, Saturating},
		SaturatedConversion,
	};

	pub type BalanceOf<T, A> = <T as InspectFungible<A>>::Balance;
	pub type DepositCurrencyHoldReasonOf<T> =
		<<T as Config>::DepositCurrency as frame_support::traits::fungible::InspectHold<
			<T as frame_system::Config>::AccountId,
		>>::Reason;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The currency used for storage deposits.
		type DepositCurrency: MutateHold<Self::AccountId>;
		/// The currency used as collateral for minting bonded tokens.
		type CollateralCurrency: Mutate<Self::AccountId>;
		/// Implementation of creating and managing new fungibles
		type Fungibles: CreateFungibles<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ DestroyFungibles<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ FungiblesMetadata<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
		/// The maximum number of currencies allowed for a single pool.
		#[pallet::constant]
		type MaxCurrencies: Get<u32> + TypeInfo;
		/// The deposit required for each bonded currency.
		#[pallet::constant]
		type DepositPerCurrency: Get<BalanceOf<Self::DepositCurrency, Self::AccountId>>;
		/// Who can create new bonded currency pools.
		type PoolCreateOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;
		/// The type used for pool ids
		type PoolId: EncodeLike
			+ Decode
			+ TypeInfo
			+ Clone
			+ MaxEncodedLen
			+ From<Self::AssetId>
			+ Into<Self::AccountId>
			+ Into<DepositCurrencyHoldReasonOf<Self>>;
		/// Type of an asset id in the Fungibles implementation
		type AssetId: AssetId + Default + EnsureAddAssign + One;
		/// The balance of assets in the Fungibles implementation
		type Balance: Balance;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Bonded Currency Swapping Pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub(crate) type Pools<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::PoolId,
		PoolDetails<T::AccountId, T::AssetId, Curve, T::MaxCurrencies>,
		OptionQuery,
	>;

	/// The asset id to be used for the next pool creation.
	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub(crate) type NextAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new bonded token pool has been initiated. [pool_id]
		PoolCreated(T::AccountId),
		/// Trading locks on a pool have been removed. [pool_id]
		Unlocked(T::AccountId),
		/// Trading locks on a pool have been set or changed. [pool_id]
		LockSet(T::AccountId),
		/// A bonded token pool has been moved to destroying state. [pool_id]
		DestructionStarted(T::AccountId),
		/// A bonded token pool has been fully destroyed and all collateral and deposits have been refunded. [pool_id]
		Destroyed(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The number of bonded currencies on a new pool is either lower than 1 or greater than MaxCurrencies.
		CurrenciesOutOfBounds,
		/// A token swap cannot be executed due to a lock placed on this operation.
		Locked,
		/// The pool id is not currently registered.
		PoolUnknown,
		/// The pool has no associated bonded currency with the given index.
		IndexOutOfBounds,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))] // TODO: properly configure weights
		pub fn create_pool(
			origin: OriginFor<T>,
			curve: Curve,
			currencies: BoundedVec<TokenMeta<T::Balance>, T::MaxCurrencies>,
			frozen: bool,
			// currency_admin: Option<T::AccountId> TODO: use this to set currency admin
		) -> DispatchResultWithPostInfo {
			// ensure origin is PoolCreateOrigin
			let who = T::PoolCreateOrigin::ensure_origin(origin)?;

			ensure!(
				(2..=T::MaxCurrencies::get().saturated_into()).contains(&currencies.len()),
				Error::<T>::CurrenciesOutOfBounds
			);

			let mut asset_id = Self::next_asset_id();
			let pool_id = T::PoolId::from(asset_id.clone());

			T::DepositCurrency::hold(
				&pool_id.clone().into(), // TODO: just assumed that you can use a pool id as hold reason, not sure that's true though
				&who,
				T::DepositPerCurrency::get()
					.saturating_mul(currencies.len().saturated_into())
					.saturated_into(),
			)?;

			let mut currency_ids = BoundedVec::with_bounded_capacity(currencies.len());

			for (idx, entry) in currencies.iter().enumerate() {
				T::Fungibles::create(asset_id.clone(), pool_id.clone().into(), false, entry.min_balance)?;
				currency_ids[idx] = asset_id.clone();
				asset_id.ensure_add_assign(T::AssetId::one())?;

				T::Fungibles::set(
					asset_id.clone(),
					&pool_id.clone().into(),
					entry.name.clone(),
					entry.symbol.clone(),
					entry.decimals,
				)?;

				// TODO: use fungibles::roles::ResetTeam to update currency admin
			}

			<NextAssetId<T>>::put(asset_id);

			<Pools<T>>::set(
				pool_id,
				Some(PoolDetails::new(who.clone(), curve, currency_ids, !frozen)),
			);

			// Emit an event.
			Self::deposit_event(Event::PoolCreated(who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}
}
