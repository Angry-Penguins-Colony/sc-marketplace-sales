use apc_sales::{auction::Auction, EmptyContract};
use multiversx_sc::types::{BoxedBytes, TokenIdentifier};
use multiversx_sc_scenario::{managed_biguint, managed_token_id_wrapped, rust_biguint, DebugApi};

use crate::helpers;

#[test]
fn create_auction() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE_TOKEN: &[u8] = b"ITEM-a1a1a1";
    const PRICE_NONCE: u64 = 600u64;

    const SELL_TOKEN: &[u8] = b"HAT-ffffff";
    const SELL_NONCE: u64 = 2;

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        SELL_TOKEN,
        SELL_NONCE,
        &rust_biguint!(1),
        &BoxedBytes::empty(),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            SELL_TOKEN,
            SELL_NONCE,
            &rust_biguint!(1),
            |sc| {
                let auction_id = sc.create_auction(
                    managed_token_id_wrapped!(PRICE_TOKEN),
                    PRICE_NONCE,
                    managed_biguint!(10),
                    0,
                );

                assert_eq!(
                    sc.auctions(auction_id).get(),
                    Auction {
                        price_token_identifier: managed_token_id_wrapped!(PRICE_TOKEN),
                        price_token_nonce: PRICE_NONCE,
                        price: managed_biguint!(10),
                        start_timestamp: 0,
                        sell_token: TokenIdentifier::<DebugApi>::from_esdt_bytes(SELL_TOKEN),
                        sell_nonce: SELL_NONCE
                    }
                );
            },
        )
        .assert_ok();
}
