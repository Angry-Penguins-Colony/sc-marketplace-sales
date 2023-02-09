use apc_sales::{
    EmptyContract, ERR_INVALID_PAYMENT_WRONG_AMOUNT_SENT, ERR_INVALID_PAYMENT_WRONG_NONCE_SENT,
    ERR_INVALID_PAYMENT_WRONG_TOKEN_SENT, ERR_NOT_ENOUGHT_ITEMS, ERR_SALE_IS_NOT_OPENED_YET,
};
use multiversx_sc::types::BoxedBytes;
use multiversx_sc_scenario::rust_biguint;

use crate::helpers;

#[test]
fn buy_one_successful() {
    buy_n_succesfully(1);
}

#[test]
fn buy_two_successful() {
    buy_n_succesfully(2);
}

#[test]
fn buy_fail_if_locked() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const QUANTITY: u64 = 1;
    const START_TIMESTAMP: u64 = 10;
    const NOW_TIMESTAMP: u64 = 5;

    setup.create_default_auction(PRICE, START_TIMESTAMP, QUANTITY);
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE * QUANTITY));

    setup.blockchain_wrapper.set_block_timestamp(NOW_TIMESTAMP);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE * QUANTITY),
            |sc| {
                sc.buy(QUANTITY);
            },
        )
        .assert_user_error(ERR_SALE_IS_NOT_OPENED_YET);
}

#[test]
fn buy_fail_wrong_amount_sent() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const EXPECTED_PRICE: u64 = 50;
    const SEND_PRICE: u64 = EXPECTED_PRICE / 2;
    const QUANTITY: u64 = 1;

    setup.create_default_auction(EXPECTED_PRICE, 0, QUANTITY);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(SEND_PRICE),
            |sc| {
                sc.buy(QUANTITY);
            },
        )
        .assert_user_error(ERR_INVALID_PAYMENT_WRONG_AMOUNT_SENT);
}

#[test]
fn buy_fail_wrong_token_sent() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const QUANTITY: u64 = 1;

    const SELL_TOKEN: &[u8] = b"SELL-ffffff";
    const SELL_NONCE: u64 = 1;

    setup.create_default_auction(PRICE, 0, QUANTITY);
    setup.blockchain_wrapper.set_nft_balance(
        &setup.user_address,
        SELL_TOKEN,
        SELL_NONCE,
        &rust_biguint!(PRICE),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.user_address,
            &setup.contract_wrapper,
            SELL_TOKEN,
            SELL_NONCE,
            &rust_biguint!(PRICE),
            |sc| sc.buy(QUANTITY),
        )
        .assert_user_error(ERR_INVALID_PAYMENT_WRONG_TOKEN_SENT);
}

#[test]
fn buy_fail_wrong_nonce_sent() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const QUANTITY: u64 = 1;

    const SELL_TOKEN: &[u8] = b"SELL-ffffff";

    const SELL_NONCE_SENT: u64 = 1;
    const SELL_NONCE_EXPECTED: u64 = 2;

    setup.create_auction(SELL_TOKEN, SELL_NONCE_EXPECTED, PRICE, 0, QUANTITY);
    setup.blockchain_wrapper.set_nft_balance(
        &setup.user_address,
        SELL_TOKEN,
        SELL_NONCE_SENT,
        &rust_biguint!(PRICE),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.user_address,
            &setup.contract_wrapper,
            SELL_TOKEN,
            SELL_NONCE_SENT,
            &rust_biguint!(PRICE),
            |sc| sc.buy(QUANTITY),
        )
        .assert_user_error(ERR_INVALID_PAYMENT_WRONG_NONCE_SENT);
}

#[test]
fn buy_fail_if_not_enough_quantity_remaining() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const AVAILABLE_QUANTITY: u64 = 2;
    const BUY_QUANTITY: u64 = 3;

    assert_eq!(BUY_QUANTITY > AVAILABLE_QUANTITY, true);

    setup.create_default_auction(PRICE, 0, AVAILABLE_QUANTITY);
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE * BUY_QUANTITY));

    // buy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE * BUY_QUANTITY),
            |sc| {
                sc.buy(BUY_QUANTITY);
            },
        )
        .assert_user_error(ERR_NOT_ENOUGHT_ITEMS);
}

fn buy_n_succesfully(quantity: u64) {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;

    setup.create_default_auction(PRICE, 0, quantity);
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE * quantity));

    // buy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(PRICE * quantity),
            |sc| {
                sc.buy(quantity);
            },
        )
        .assert_ok();

    // "The buying user should have spend all his money."
    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.user_address, &rust_biguint!(0));

    // "The buying user should have receive its SFT"
    setup.blockchain_wrapper.check_nft_balance(
        &setup.user_address,
        crate::helpers::DEFAULT_AUCTION_SELL_TOKEN,
        crate::helpers::DEFAULT_AUCTION_SELL_NONCE,
        &rust_biguint!(quantity),
        Option::Some(&BoxedBytes::empty()),
    );
}
