use super::*;
use crate::mock::{new_test_ext, KittiesModule, Origin, Test};
use crate::Error;
use frame_support::dispatch::DispatchResult;
use frame_support::{assert_noop, assert_ok};

/// create new kitty
fn new_kitty(account_id: u64) -> DispatchResult {
	KittiesModule::create(Origin::signed(account_id))
}

#[test]
fn create_success() {
	new_test_ext().execute_with(|| {
		assert_ok!(new_kitty(1));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
	});
}

#[test]
fn create_fail_max_count_overflow() {
	new_test_ext().execute_with(|| {
		NextKittyId::<Test>::put(u32::max_value());
		assert_noop!(new_kitty(1), Error::<Test>::KittiesCountOverflow);
	});
}

#[test]
fn create_failed_token_not_enough() {
	new_test_ext().execute_with(|| {
		assert_noop!(new_kitty(99), Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn transfer_success() {
	new_test_ext().execute_with(|| {
		let _ = new_kitty(1);
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
	});
}

#[test]
fn transfer_fail_not_owner() {
	new_test_ext().execute_with(|| {
		let _ = new_kitty(1);
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn breed_success() {
	new_test_ext().execute_with(|| {
		let _ = new_kitty(1);
		let _ = new_kitty(1);

		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
		assert_eq!(KittiesModule::next_kitty_id(), 3);
	});
}

#[test]
fn breed_fail_same_kitty_id() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 1), Error::<Test>::SameKittyId);
	});
}

#[test]
fn breed_fail_invalid_kittyid() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn breed_fail_count_overflow() {
	new_test_ext().execute_with(|| {
		let _ = new_kitty(1);
		let _ = new_kitty(1);

		NextKittyId::<Test>::put(u32::max_value());

		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::KittiesCountOverflow);
	});
}