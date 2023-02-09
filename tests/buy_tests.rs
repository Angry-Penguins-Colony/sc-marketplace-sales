use apc_sales::EmptyContract;
use multiversx_sc::types::BoxedBytes;
use multiversx_sc_scenario::rust_biguint;

use crate::helpers;

#[test]
fn buy_one_successful() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 100;

    setup.create_default_auction(PRICE, 0, 5);
    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.user_address, &rust_biguint!(PRICE));

    // buy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| {
                sc.buy(1);
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
        &rust_biguint!(0),
        Option::Some(&BoxedBytes::empty()),
    );
}
