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

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{
			fungible::{Inspect as InspectFungible, Mutate, MutateHold},
			fungibles::{
				metadata::Mutate as FungiblesMetadata, Create as CreateFungibles, Destroy as DestroyFungibles,
				Inspect as InspectFungibles, Mutate as MutateFungibles,
			},
			tokens::Preservation,
		},
		Hashable,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{CheckedAdd, Saturating, StaticLookup, Zero},
		ArithmeticError, SaturatedConversion,
	};

	use crate::types::{Curve, MockCurve, PoolDetails, PoolStatus, TokenMeta};

	type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source;
	type DepositCurrencyBalanceOf<T> =
		<<T as Config>::DepositCurrency as InspectFungible<<T as frame_system::Config>::AccountId>>::Balance;
	type DepositCurrencyHoldReasonOf<T> =
		<<T as Config>::DepositCurrency as frame_support::traits::fungible::InspectHold<
			<T as frame_system::Config>::AccountId,
		>>::Reason;
	type CollateralCurrencyBalanceOf<T> =
		<<T as Config>::CollateralCurrency as InspectFungible<<T as frame_system::Config>::AccountId>>::Balance;
	type FungiblesBalanceOf<T> =
		<<T as Config>::Fungibles as InspectFungibles<<T as frame_system::Config>::AccountId>>::Balance;
	type FungiblesAssetIdOf<T> =
		<<T as Config>::Fungibles as InspectFungibles<<T as frame_system::Config>::AccountId>>::AssetId;

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
		type Fungibles: CreateFungibles<Self::AccountId>
			+ DestroyFungibles<Self::AccountId>
			+ FungiblesMetadata<Self::AccountId>
			+ MutateFungibles<Self::AccountId>;
		/// The maximum number of currencies allowed for a single pool.
		#[pallet::constant]
		type MaxCurrencies: Get<u32> + TypeInfo;
		/// The deposit required for each bonded currency.
		#[pallet::constant]
		type DepositPerCurrency: Get<DepositCurrencyBalanceOf<Self>>;
		/// Who can create new bonded currency pools.
		type PoolCreateOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;
		/// The type used for pool ids
		type PoolId: Parameter
			+ MaxEncodedLen
			+ From<[u8; 32]>
			+ Into<Self::AccountId>
			+ Into<DepositCurrencyHoldReasonOf<Self>>;
	}

	type CurveParameterType = u32;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Bonded Currency Swapping Pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub(crate) type Pools<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::PoolId,
		PoolDetails<T::AccountId, FungiblesAssetIdOf<T>, Curve<CurveParameterType>, T::MaxCurrencies>,
		OptionQuery,
	>;

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
		/// The cost or returns for a mint, burn, or swap operation is outside the user-defined slippage tolerance.
		Slippage,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		FungiblesBalanceOf<T>: TryInto<CollateralCurrencyBalanceOf<T>>,
	{
		#[pallet::call_index(0)]
		// #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))] TODO: properly configure weights
		pub fn create_pool(
			origin: OriginFor<T>,
			curve: Curve<CurveParameterType>,
			currencies: BoundedVec<TokenMeta<FungiblesBalanceOf<T>, FungiblesAssetIdOf<T>>, T::MaxCurrencies>,
			frozen: bool,
			// currency_admin: Option<AccountIdLookupOf<T>> TODO: use this to set currency admin
		) -> DispatchResultWithPostInfo {
			// ensure origin is PoolCreateOrigin
			let who = T::PoolCreateOrigin::ensure_origin(origin)?;

			ensure!(
				(2..=T::MaxCurrencies::get().saturated_into()).contains(&currencies.len()),
				Error::<T>::CurrenciesOutOfBounds
			);

			let currency_ids = BoundedVec::truncate_from(currencies.iter().map(|c| c.id.clone()).collect());

			let pool_id = T::PoolId::from(currency_ids.blake2_256());

			T::DepositCurrency::hold(
				&pool_id.clone().into(), // TODO: just assumed that you can use a pool id as hold reason, not sure that's true though
				&who,
				T::DepositPerCurrency::get()
					.saturating_mul(currencies.len().saturated_into())
					.saturated_into(),
			)?;

			for entry in currencies {
				let asset_id = entry.id.clone();

				// create new assset class; fail if it already exists
				T::Fungibles::create(asset_id.clone(), pool_id.clone().into(), false, entry.min_balance)?;

				// set metadata for new asset class
				T::Fungibles::set(
					asset_id,
					&pool_id.clone().into(),
					entry.name.clone(),
					entry.symbol.clone(),
					entry.decimals,
				)?;

				// TODO: use fungibles::roles::ResetTeam to update currency admin
			}

			<Pools<T>>::set(
				pool_id,
				Some(PoolDetails::new(who.clone(), curve, currency_ids, !frozen)),
			);

			// Emit an event.
			Self::deposit_event(Event::PoolCreated(who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::call_index(1)]
		pub fn mint_into(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			currency_idx: u32,
			amount_to_mint: FungiblesBalanceOf<T>,
			max_cost: CollateralCurrencyBalanceOf<T>,
			beneficiary: AccountIdLookupOf<T>,
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			let beneficiary = T::Lookup::lookup(beneficiary)?;

			let pool_details = <Pools<T>>::get(pool_id.clone()).ok_or(Error::<T>::PoolUnknown)?;

			let mint_enabled = match pool_details.state {
				// if mint is locked, then operation is priviledged
				PoolStatus::Frozen(locks) => locks.allow_mint || signer == pool_details.creator,
				PoolStatus::Active => true,
				_ => false,
			};
			ensure!(mint_enabled, Error::<T>::Locked);

			let currency_idx_usize: usize = currency_idx.saturated_into();

			// get id of the currency we want to mint
			// this also serves as a validation of the currency_idx parameter
			let mint_currency_id = pool_details
				.bonded_currencies
				.get(currency_idx_usize)
				.ok_or(Error::<T>::IndexOutOfBounds)?;

			let total_issuances: Vec<FungiblesBalanceOf<T>> = pool_details
				.bonded_currencies
				.iter()
				.map(|id| T::Fungibles::total_issuance(id.clone()))
				.collect();

			let cost = Self::get_cost(pool_details.curve, &amount_to_mint, total_issuances, currency_idx_usize)?;

			// fail if cost > max_cost
			ensure!(!cost.gt(&max_cost), Error::<T>::Slippage);

			// withdraw the collateral and put it in the deposit account
			T::CollateralCurrency::transfer(&signer, &pool_id.into(), cost, Preservation::Preserve)?;

			// mint tokens into beneficiary account
			T::Fungibles::mint_into(mint_currency_id.clone(), &beneficiary, amount_to_mint)?;

			// TODO: apply lock if pool_details.transferable != true

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T>
	where
		FungiblesBalanceOf<T>: TryInto<CollateralCurrencyBalanceOf<T>>,
	{
		pub fn get_cost(
			curve: Curve<CurveParameterType>,
			amount_to_mint: &FungiblesBalanceOf<T>,
			mut total_issuances: Vec<FungiblesBalanceOf<T>>,
			mint_into_idx: usize,
		) -> Result<CollateralCurrencyBalanceOf<T>, ArithmeticError> {
			// calculate parameters for bonding curve
			// we've checked the vector length before
			let active_issuance_pre = total_issuances.swap_remove(mint_into_idx);
			let active_issuance_post = active_issuance_pre
				.checked_add(amount_to_mint)
				.ok_or(ArithmeticError::Overflow)?;
			let passive_issuance: FungiblesBalanceOf<T> = total_issuances
				.iter()
				.fold(Zero::zero(), |sum, x| sum.saturating_add(*x));

			// match curve implementation
			let curve_impl = match curve {
				Curve::LinearRatioCurve(_) => MockCurve::new(),
			};

			let cost = curve_impl.calculate_cost(active_issuance_pre, active_issuance_post, passive_issuance);

			// Try conversion to Collateral Balance type
			return cost.try_into().map_err(|_| ArithmeticError::Overflow);
		}
	}
}
