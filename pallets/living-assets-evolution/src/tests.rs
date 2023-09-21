use crate::{mock::*, types::TokenUriOf, CollectionId, Error, Event};
use frame_support::{assert_noop, assert_ok};

/// Utility function to create a collection and return its ID
fn create_collection(owner: u64) -> CollectionId {
	let collection_id = LivingAssets::collection_counter();
	assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(owner)));
	collection_id
}

#[test]
fn owner_of_inexistent_collection() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
	});
}

#[test]
fn create_collection_works() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(1));
		let collection_id: CollectionId = 1;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(2)));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(2));
	});
}

#[test]
fn counter_of_collection_increases() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssets::collection_counter(), 0);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(LivingAssets::collection_counter(), 1);
	})
}

#[test]
fn create_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let collection_id = create_collection(1);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::CollectionCreated { collection_id, owner: 1 }.into());
	});
}

#[test]
fn mint_with_external_uri_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_ok!(LivingAssets::mint_with_external_uri(
			RuntimeOrigin::signed(1),
			collection_id,
			0,
			1,
			token_uri.clone()
		));

		assert_eq!(LivingAssets::asset_owner(collection_id, 0), Some(1));
		assert_eq!(LivingAssets::asset_metadata(collection_id, 0), Some(token_uri.clone()));

		System::assert_has_event(
			Event::ExternalUriSet { collection_id, slot: 0, token_uri }.into(),
		);
		System::assert_has_event(Event::Minted { collection_id, slot: 0, to: 1 }.into());
	});
}

#[test]
fn mint_with_external_uri_non_owner() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(2),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn mint_with_external_uri_collection_does_not_exist() {
	new_test_ext().execute_with(|| {
		// simply use the collection counter as collection id, do not create the collection
		let collection_id = LivingAssets::collection_counter();

		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(1),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::CollectionDoesNotExist
		);
	});
}

#[test]
fn mint_with_external_uri_asset_already_minted() {
	new_test_ext().execute_with(|| {
		let collection_id = LivingAssets::collection_counter();
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_ok!(LivingAssets::mint_with_external_uri(
			RuntimeOrigin::signed(1),
			collection_id,
			0,
			1,
			token_uri.clone()
		));

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(1),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::AlreadyMinted
		);
	});
}
