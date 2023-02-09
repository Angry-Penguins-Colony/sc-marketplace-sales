use apc_sales::{EmptyContract, ERR_SALE_IS_NOT_OPENED_YET};
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
