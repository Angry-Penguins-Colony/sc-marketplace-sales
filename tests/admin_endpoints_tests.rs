use apc_sales::{
    EmptyContract, ERR_INVALID_AUCTION_ID, ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH,
    ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH, ERR_RETIRING_TOO_MUCH_TOKENS, STARTING_AUCTION_ID,
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

            assert_eq!(auction.auction.current_quantity, REMAINING_QUANTITY);
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
fn retire_tokens_fail_if_too_much_tokens() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const PRICE: u64 = 1;
    const BUY_QUANTITY: u64 = 1;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE * BUY_QUANTITY));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE * BUY_QUANTITY),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.retire_token_from_auction(
                    STARTING_AUCTION_ID,
                    &managed_biguint!(INITIAL_QUANTITY),
                )
            },
        )
        .assert_user_error(ERR_RETIRING_TOO_MUCH_TOKENS);
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

            assert_eq!(auction.auction.current_quantity, NEW_QUANTITY);
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

#[test]
fn withdraw_works_with_egld() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const PRICE: u64 = 1;

    setup.create_default_auction_buyable_in_egld(PRICE, 0, INITIAL_QUANTITY);

    // 1. user buy
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.owner_address, &rust_biguint!(0));

    // 2. call withdraw
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.withdraw_balance();
            },
        )
        .assert_ok();

    // 3. assert balance
    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.owner_address, &rust_biguint!(PRICE));
}

#[test]
fn withdraw_works_with_esdt() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const PRICE: u64 = 1;

    const INPUT_TOKEN_ID: &[u8] = b"INPUT-aaaaaa";
    const INPUT_TOKEN_NONCE: u64 = 1;

    const OUTPUT_TOKEN_ID: &[u8] = b"OUTPUT-ffffff";
    const OUTPUT_TOKEN_NONCE: u64 = 1;

    setup.create_auction_buyable_in_esdt(
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        OUTPUT_TOKEN_ID,
        OUTPUT_TOKEN_NONCE,
        PRICE,
        0,
        INITIAL_QUANTITY,
    );

    // 2. the user buy
    setup.blockchain_wrapper.set_nft_balance(
        &setup.user_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(PRICE),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.user_address,
            &setup.contract_wrapper,
            INPUT_TOKEN_ID,
            INPUT_TOKEN_NONCE,
            &rust_biguint!(PRICE),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    // 3. withdraw
    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(0),
        Option::Some(&BoxedBytes::empty()),
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.withdraw_balance();
            },
        )
        .assert_ok();

    // 4. assert eq
    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(PRICE),
        Option::Some(&BoxedBytes::empty()),
    );
}

#[test]
fn hide_auction_works() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const PRICE: u64 = 1;
    const BUY_QUANTITY: u64 = 1;

    setup.create_default_auction_buyable_in_egld(1, 0, INITIAL_QUANTITY);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE * BUY_QUANTITY));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE * BUY_QUANTITY),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.hide_auction(STARTING_AUCTION_ID),
        )
        .assert_ok();

    // assert max_quantity == 0
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let auction_stats = sc.get_auction_stats(STARTING_AUCTION_ID);

            assert_eq!(
                auction_stats.auction.max_quantity, 0,
                "The max quantity should be 0"
            );
        })
        .assert_ok();

    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
        helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
        &rust_biguint!(INITIAL_QUANTITY - BUY_QUANTITY),
        Option::Some(&BoxedBytes::empty()),
    );
}

#[test]
fn hide_fails_if_wrong_auction_id() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.hide_auction(STARTING_AUCTION_ID),
        )
        .assert_user_error(ERR_INVALID_AUCTION_ID);
}

#[test]
fn withdraw_esdt_do_not_exceed() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const EXCEED_QUANTITY: u64 = 20;
    const PRICE: u64 = 1;

    const INPUT_TOKEN_ID: &[u8] = b"INPUT-aaaaaa";
    const INPUT_TOKEN_NONCE: u64 = 1;

    const OUTPUT_TOKEN_ID: &[u8] = b"OUTPUT-ffffff";
    const OUTPUT_TOKEN_NONCE: u64 = 1;

    setup.create_auction_buyable_in_esdt(
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        OUTPUT_TOKEN_ID,
        OUTPUT_TOKEN_NONCE,
        PRICE,
        0,
        INITIAL_QUANTITY,
    );

    // make exceed output quantity
    setup.blockchain_wrapper.set_nft_balance(
        &setup.contract_wrapper.address_ref(),
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(EXCEED_QUANTITY),
        &BoxedBytes::empty(),
    );

    // 2. the user buy
    setup.blockchain_wrapper.set_nft_balance(
        &setup.user_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(PRICE),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.user_address,
            &setup.contract_wrapper,
            INPUT_TOKEN_ID,
            INPUT_TOKEN_NONCE,
            &rust_biguint!(PRICE),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    // 3. withdraw
    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(0),
        Option::Some(&BoxedBytes::empty()),
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.withdraw_balance();
            },
        )
        .assert_ok();

    // 4. assert eq
    setup.blockchain_wrapper.check_nft_balance(
        &setup.owner_address,
        INPUT_TOKEN_ID,
        INPUT_TOKEN_NONCE,
        &rust_biguint!(PRICE),
        Option::Some(&BoxedBytes::empty()),
    );
}

/**
 * Withdraw takes all the ""input" tokens on the wallet.
 * It is an unwanted behaviour when an input token is also an output token.
 * It would bypass retireTokens and therefore break everything.
 */
#[test]
fn withdraw_egld_do_not_exceed() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const INITIAL_QUANTITY: u64 = 100;
    const PRICE: u64 = 1;
    const EXCEED_EGLD: u64 = 5;

    setup.create_default_auction_buyable_in_egld(PRICE, 0, INITIAL_QUANTITY);

    // 1. user buy
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE),
            |sc| {
                sc.buy(STARTING_AUCTION_ID);
            },
        )
        .assert_ok();

    setup.blockchain_wrapper.set_egld_balance(
        &setup.contract_wrapper.address_ref(),
        &rust_biguint!(PRICE + EXCEED_EGLD),
    );

    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.owner_address, &rust_biguint!(0));

    // 2. call withdraw
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.withdraw_balance();
            },
        )
        .assert_ok();

    // 3. assert balance
    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.owner_address, &rust_biguint!(PRICE));

    setup.blockchain_wrapper.check_egld_balance(
        &setup.contract_wrapper.address_ref(),
        &rust_biguint!(EXCEED_EGLD),
    );
}
