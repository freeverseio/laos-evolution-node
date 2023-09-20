#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod types;
pub mod weights;

use types::*;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::One;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Limit for the length of `token_uri`
		type MaxTokenUriLength: Get<u32>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Storage for the ownership of collections
	#[pallet::storage]
	#[pallet::getter(fn collection_owner)]
	pub type CollectionOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, AccountIdOf<T>, OptionQuery>;

	/// Minted assets
	#[pallet::storage]
	#[pallet::getter(fn asset_owner)]
	pub type AssetOwner<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		Slot,
		AccountIdOf<T>,
		OptionQuery,
	>;

	/// Asset metadata
	/// This will contain external URI in a raw form
	#[pallet::storage]
	#[pallet::getter(fn asset_info)]
	pub type AssetInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		Slot,
		TokenUriOf<T>,
		OptionQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, owner: AccountIdOf<T> },
		/// Asset minted
		/// [collection_id, slot, to, token_uri]
		AssetMinted {
			collection_id: CollectionId,
			slot: Slot,
			to: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
		},
		/// External URI set
		/// [collection_id, slot, token_uri]
		ExternalUriSet { collection_id: CollectionId, slot: Slot, token_uri: TokenUriOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The collection ID counter has overflowed
		CollectionIdOverflow,
		/// Collection does not exist
		CollectionDoesNotExist,
		/// Not the owner of the collection
		NoPermission,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_collection(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let collection_id = Self::collection_counter();

			CollectionOwner::<T>::insert(collection_id, who.clone());

			// Attempt to increment the collection counter by 1. If this operation
			// would result in an overflow, return early with an error
			let counter =
				collection_id.checked_add(One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
			CollectionCounter::<T>::put(counter);

			// Emit an event.
			Self::deposit_event(Event::CollectionCreated { collection_id, owner: who });

			// Return a successful DispatchResult
			Ok(())
		}

		/// Mint new asset with external URI
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn mint_with_external_uri(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			to: AccountIdOf<T>,
			slot: Slot,
			token_uri: TokenUriOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				CollectionOwner::<T>::get(collection_id) == Some(who),
				Error::<T>::NoPermission
			);

			ensure!(
				CollectionOwner::<T>::contains_key(collection_id),
				Error::<T>::CollectionDoesNotExist
			);

			AssetOwner::<T>::insert(collection_id, slot, to.clone());

			AssetInfo::<T>::insert(collection_id, slot, token_uri);

			Ok(())
		}
	}
}
