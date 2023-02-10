use apc_sales::{auction::Auction, EmptyContract, STARTING_AUCTION_ID};
use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, TokenIdentifier};
use multiversx_sc_scenario::{managed_biguint, DebugApi};

use crate::helpers;

#[test]
fn create_auction() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 10;
    const START_TIMESTAMP: u64 = 0;

    const SELL_TOKEN: &[u8] = b"HAT-ffffff";
    const SELL_NONCE: u64 = 2;
    const SELL_QUANTITY: u64 = 5;

    setup.create_auction_buyable_in_egld(
        SELL_TOKEN,
        SELL_NONCE,
        PRICE,
        START_TIMESTAMP,
        SELL_QUANTITY,
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
                    input_token_id: TokenIdentifier::<DebugApi>::from_esdt_bytes(SELL_TOKEN),
                    input_token_nonce: SELL_NONCE
                }
            );
        })
        .assert_ok();
}
