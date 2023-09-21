//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as LivingAssetsEvo;
use frame_benchmarking::v2::*;
use frame_support::traits::Get;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_collection() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create_collection(RawOrigin::Signed(caller.clone()));

		assert_eq!(CollectionOwner::<T>::get(0), Some(caller));
	}

	#[benchmark]
	fn mint_with_external_uri() {
		let caller: T::AccountId = whitelisted_caller();
		LivingAssetsEvo::<T>::create_collection(RawOrigin::Signed(caller.clone()).into()).unwrap();

		let token_uri: TokenUriOf<T> =
			vec![0; T::MaxTokenUriLength::get() as usize].try_into().unwrap();
		let slot = [0u8; 12].into();

		#[extrinsic_call]
		mint_with_external_uri(
			RawOrigin::Signed(caller.clone()),
			0,
			slot,
			caller.clone(),
			token_uri.clone(),
		);

		assert_eq!(AssetOwner::<T>::get(0, slot), Some(caller));
		assert_eq!(ExplicitTokenURI::<T>::get(0, slot), Some(token_uri));
	}

	impl_benchmark_test_suite!(LivingAssetsEvo, crate::mock::new_test_ext(), crate::mock::Test);
}
