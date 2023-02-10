use apc_sales::{
    EmptyContract, ERR_INVALID_AUCTION_ID, ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH,
    ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH, STARTING_AUCTION_ID,
};
use multiversx_sc::types::BoxedBytes;
use multiversx_sc_scenario::{managed_biguint, rust_biguint};

use crate::helpers;

#[test]
fn retire_token_should_work() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const RETIRED_QUANTITY: u64 = 5;
    const REMAINING_QUANTITY: u64 = 95;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.retire_token_from_auction(
                    STARTING_AUCTION_ID,
                    &managed_biguint!(RETIRED_QUANTITY),
                )
            },
        )
        .assert_ok();

    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(RETIRED_QUANTITY),
        Option::Some(&BoxedBytes::empty()),
    );

    setup.blockchain_wrapper.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(REMAINING_QUANTITY),
        Option::Some(&BoxedBytes::empty()),
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let auction = sc.get_auction_stats(STARTING_AUCTION_ID);

            assert_eq!(auction.remaining_output_items, REMAINING_QUANTITY);
        })
        .assert_ok();
}

#[test]
fn retire_token_fails_if_bad_auction_id() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.retire_token_from_auction(STARTING_AUCTION_ID, &managed_biguint!(1)),
        )
        .assert_user_error(ERR_INVALID_AUCTION_ID);
}

#[test]
fn add_tokens_should_work() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const ADDED_QUANTITY: u64 = 5;
    const NEW_QUANTITY: u64 = INITIAL_QUANTITY + ADDED_QUANTITY;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(ADDED_QUANTITY),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
            helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
            &rust_biguint!(ADDED_QUANTITY),
            |sc| sc.add_token_to_auction(STARTING_AUCTION_ID),
        )
        .assert_ok();

    setup.blockchain_wrapper.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(NEW_QUANTITY),
        Option::Some(&BoxedBytes::empty()),
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let auction = sc.get_auction_stats(STARTING_AUCTION_ID);

            assert_eq!(auction.remaining_output_items, NEW_QUANTITY);
        })
        .assert_ok();
}

#[test]
fn add_tokens_fails_if_send_wrong_token_identifier() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const ADDED_QUANTITY: u64 = 5;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        b"WRONG-bbbbbb",
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(ADDED_QUANTITY),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            b"WRONG-bbbbbb",
            helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
            &rust_biguint!(ADDED_QUANTITY),
            |sc| sc.add_token_to_auction(STARTING_AUCTION_ID),
        )
        .assert_user_error(ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH);
}

#[test]
fn add_tokens_fails_if_send_wrong_token_nonce() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const ADDED_QUANTITY: u64 = 5;

    const WRONG_NONCE: u64 = helpers::DEFAULT_AUCTION_OUTPUT_NONCE + 1;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        WRONG_NONCE,
        &rust_biguint!(ADDED_QUANTITY),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
            WRONG_NONCE,
            &rust_biguint!(ADDED_QUANTITY),
            |sc| sc.add_token_to_auction(STARTING_AUCTION_ID),
        )
        .assert_user_error(ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH);
}

#[test]
fn add_tokens_fails_if_bad_id() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const ADDED_QUANTITY: u64 = 5;

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(ADDED_QUANTITY),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
            helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
            &rust_biguint!(ADDED_QUANTITY),
            |sc| sc.add_token_to_auction(STARTING_AUCTION_ID),
        )
        .assert_user_error(ERR_INVALID_AUCTION_ID);
}
