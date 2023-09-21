//! Types used in the pallet

use codec::{Decode, Encode, MaxEncodedLen};
use sp_core::U256;
use sp_runtime::BoundedVec;

/// Collection id type
pub type CollectionId = u64;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// AssetId type
/// every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
pub type AssetId = U256;

/// Slot type - 96-bit unsigned integer
#[derive(
	Encode, Decode, Debug, Default, Clone, Copy, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo,
)]
pub struct Slot(pub [u8; 12]);

impl Into<[u8; 12]> for Slot {
	fn into(self) -> [u8; 12] {
		self.0
	}
}

impl From<[u8; 12]> for Slot {
	fn from(bytes: [u8; 12]) -> Self {
		Self(bytes)
	}
}
