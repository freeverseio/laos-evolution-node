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

use sp_core::H160;
use sp_runtime::traits::Convert;

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
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// Converts [`AccountId`] to [`H160`]
		type AccountIdToH160: Convert<AccountIdOf<Self>, H160>;
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

	/// Token URI which can override the default URI scheme and set explicitly
	/// This will contain external URI in a raw form
	#[pallet::storage]
	#[pallet::getter(fn asset_metadata)]
	pub type ExplicitTokenURI<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		Slot,
		TokenUriOf<T>,
		OptionQuery,
	>;

	/// Events for this pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, owner: AccountIdOf<T> },
		/// Asset minted
		/// [collection_id, slot, to, token_uri]
		Minted { collection_id: CollectionId, slot: Slot, to: AccountIdOf<T>, asset_id: AssetId },
		/// External URI set
		/// [collection_id, slot, token_uri]
		ExplicitTokenURISet { collection_id: CollectionId, slot: Slot, token_uri: TokenUriOf<T> },
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
		/// [`Slot`] is already minted
		AlreadyMinted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The `create_collection` extrinsic allows users to create a new collection.
		///
		/// # Parameters
		///
		/// - `origin`: The origin account sending the extrinsic, which will be set as the owner of the new collection.
		///
		/// # Storage Changes
		///
		/// - [`CollectionOwner`](`CollectionOwner`): Inserts a new mapping from the generated `collection_id` to the `origin` account.
		/// - [`CollectionCounter`](`CollectionCounter`): Updates the counter for the next available `collection_id`.
		///
		/// # Events
		///
		/// Emits a [`CollectionCreated`](`Event::<T>::CollectionCreated`) event upon successful execution.
		///
		/// # Errors
		///
		/// - Returns [`CollectionIdOverflow`](`Error::<T>::CollectionIdOverflow`) if incrementing the `collection_id` counter would result in an overflow.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_collection())]
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
		///
		/// This function performs the minting of a new asset with setting its external URI.
		///
		/// # Errors
		///
		///  This function returns a dispatch error in the following cases:
		///
		/// * [`NoPermission`](`Error::<T>::NoPermission`) - if the caller is not the owner of the collection
		/// * [`CollectionDoesNotExist`](`Error::<T>::CollectionDoesNotExist`) - if the collection does not exist
		/// * [`AlreadyMinted`](`Error::<T>::AlreadyMinted`) - if the asset is already minted
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::mint_with_external_uri())]
		pub fn mint_with_external_uri(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			slot: Slot,
			to: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				CollectionOwner::<T>::contains_key(collection_id),
				Error::<T>::CollectionDoesNotExist
			);

			ensure!(
				CollectionOwner::<T>::get(collection_id) == Some(who),
				Error::<T>::NoPermission
			);

			ensure!(AssetOwner::<T>::get(collection_id, slot).is_none(), Error::<T>::AlreadyMinted);

			AssetOwner::<T>::insert(collection_id, slot, to.clone());

			ExplicitTokenURI::<T>::insert(collection_id, slot, token_uri.clone());

			// compose asset_id	from slot and owner
			let asset_id = Self::slot_and_owner_to_asset_id((slot, to.clone()));

			Self::deposit_event(Event::Minted { collection_id, slot, to, asset_id });
			Self::deposit_event(Event::ExplicitTokenURISet { collection_id, slot, token_uri });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	// Utility functions
	/// A struct responsible for converting `Slot` and `AccountId` to `AssetId`
	///
	/// Every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
	fn slot_and_owner_to_asset_id(slot_and_owner: (Slot, AccountIdOf<T>)) -> AssetId {
		let (slot, owner) = slot_and_owner;

		let mut bytes = [0u8; 32];

		let slot_bytes: [u8; 12] = slot.into();

		// NOTE: this will panic at runtime if two arrays overlap, we should see if there is a safer way to do this
		bytes[..12].copy_from_slice(&slot_bytes);

		let h160 = T::AccountIdToH160::convert(owner);
		let account_id_bytes = h160.as_fixed_bytes();

		bytes[12..].copy_from_slice(account_id_bytes);
		AssetId::from(bytes)
	}
}
