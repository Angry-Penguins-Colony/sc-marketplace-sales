use apc_sales::{auction::Auction, EmptyContract, STARTING_AUCTION_ID};
use multiversx_sc::types::{BoxedBytes, TokenIdentifier};
use multiversx_sc_scenario::{managed_biguint, managed_token_id_wrapped, rust_biguint, DebugApi};

use crate::helpers;

#[test]
fn create_auction() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE_TOKEN: &[u8] = b"ITEM-a1a1a1";
    const PRICE_NONCE: u64 = 600u64;
    const PRICE: u64 = 10;
    const START_TIMESTAMP: u64 = 0;

    const SELL_TOKEN: &[u8] = b"HAT-ffffff";
    const SELL_NONCE: u64 = 2;
    const SELL_QUANTITY: u64 = 5;

    setup.create_auction(
        SELL_TOKEN,
        SELL_NONCE,
        PRICE_TOKEN,
        PRICE_NONCE,
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
                    price_token_identifier: managed_token_id_wrapped!(PRICE_TOKEN),
                    price_token_nonce: PRICE_NONCE,
                    price: managed_biguint!(10),
                    start_timestamp: 0,
                    sell_token: TokenIdentifier::<DebugApi>::from_esdt_bytes(SELL_TOKEN),
                    sell_nonce: SELL_NONCE
                }
            );
        })
        .assert_ok();
}
