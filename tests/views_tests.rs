use crate::helpers;
use apc_sales::{
    auction::{Auction, AuctionStats},
    EmptyContract, ERR_INVALID_AUCTION_ID, STARTING_AUCTION_ID,
};
use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, TokenIdentifier};
use multiversx_sc_scenario::{managed_biguint, DebugApi};

#[test]
fn view_get_auction_fail_if_invalid_id() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.get_auction(0);
        })
        .assert_user_error(ERR_INVALID_AUCTION_ID);
}

#[test]
fn view_get_auction_works() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 1;
    const START_TIMESTAMP: u64 = 4;
    const INITIAL_QUANTITY: u64 = 100;

    setup.create_default_auction_buyable_in_egld(PRICE, START_TIMESTAMP, INITIAL_QUANTITY);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let actual_auction_stats = sc.get_auction(STARTING_AUCTION_ID);

            let expected_auction_stats = AuctionStats {
                auction: Auction {
                    output_token_id: TokenIdentifier::<DebugApi>::from_esdt_bytes(
                        helpers::DEFAULT_AUCTION_OUTPUT_TOKEN,
                    ),
                    output_token_nonce: helpers::DEFAULT_AUCTION_OUTPUT_NONCE,
                    price: managed_biguint!(PRICE),
                    start_timestamp: START_TIMESTAMP,
                    input_token_id: EgldOrEsdtTokenIdentifier::egld(),
                    input_token_nonce: 0,
                },
                remaining_output_items: managed_biguint!(INITIAL_QUANTITY),
            };

            assert_eq!(actual_auction_stats, expected_auction_stats);
        })
        .assert_ok();
}
