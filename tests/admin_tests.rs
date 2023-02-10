use apc_sales::{
    auction::Auction, EmptyContract, ERR_CREATE_AUCTION_BAD_EGLD_NONCE, STARTING_AUCTION_ID,
};
use multiversx_sc::types::{BoxedBytes, EgldOrEsdtTokenIdentifier, TokenIdentifier};
use multiversx_sc_scenario::{managed_biguint, rust_biguint, DebugApi};

use crate::helpers;

#[test]
fn create_auction() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 10;
    const START_TIMESTAMP: u64 = 0;

    const OUTPUT_TOKEN_ID: &[u8] = b"HAT-ffffff";
    const OUTPUT_TOKEN_NONCE: u64 = 2;
    const QUANTITY: u64 = 5;

    setup.create_auction_buyable_in_egld(
        OUTPUT_TOKEN_ID,
        OUTPUT_TOKEN_NONCE,
        PRICE,
        START_TIMESTAMP,
        QUANTITY,
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.next_auction_id().get(), STARTING_AUCTION_ID + 1);

            assert_eq!(
                sc.auctions(STARTING_AUCTION_ID).get(),
                Auction {
                    output_token_id: EgldOrEsdtTokenIdentifier::egld(),
                    output_token_nonce: 0,
                    price: managed_biguint!(PRICE),
                    start_timestamp: START_TIMESTAMP,
                    input_token_id: TokenIdentifier::<DebugApi>::from_esdt_bytes(OUTPUT_TOKEN_ID),
                    input_token_nonce: OUTPUT_TOKEN_NONCE
                }
            );
        })
        .assert_ok();
}

#[test]
fn create_auction_fails_if_egld_nonce_is_wrong() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN_ID,
        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN_NONCE,
        &rust_biguint!(5),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            helpers::DEFAULT_AUCTION_OUTPUT_TOKEN_ID,
            helpers::DEFAULT_AUCTION_OUTPUT_TOKEN_NONCE,
            &rust_biguint!(0),
            |sc| {
                sc.create_auction(EgldOrEsdtTokenIdentifier::egld(), 1, managed_biguint!(1), 0);
            },
        )
        .assert_user_error(ERR_CREATE_AUCTION_BAD_EGLD_NONCE);
}
