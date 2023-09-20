//! Types used in the pallet

use sp_runtime::BoundedVec;

/// Collection id type
pub type CollectionId = u64;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Slot type
pub type Slot = u64;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;
