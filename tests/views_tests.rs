use crate::helpers;
use apc_sales::{
    auction::{Auction, AuctionStats},
    EmptyContract, ERR_INVALID_AUCTION_ID, STARTING_AUCTION_ID,
};
use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, TokenIdentifier};
use multiversx_sc_scenario::{
    managed_biguint, managed_token_id, managed_token_id_wrapped, rust_biguint, DebugApi,
};

#[test]
fn view_get_auction_fail_if_invalid_id() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.get_auction_stats(0);
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
            let actual_auction_stats = sc.get_auction_stats(STARTING_AUCTION_ID);

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
                    max_quantity: managed_biguint!(INITIAL_QUANTITY),
                    current_quantity: managed_biguint!(INITIAL_QUANTITY),
                },
                id: STARTING_AUCTION_ID,
            };

            assert_eq!(actual_auction_stats, expected_auction_stats);
        })
        .assert_ok();
}

struct UnmanagedAuction<'a> {
    input_token_id: &'a [u8],
    input_token_nonce: u64,
    output_token_id: &'a [u8],
    output_token_nonce: u64,
    price: u64,
    start_timestamp: u64,
    quantity: u64,
}

#[test]
fn view_get_all_auctions_works() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    let expected_auctions = [
        UnmanagedAuction {
            input_token_id: b"INPUT-aaaaaa",
            input_token_nonce: 1,
            output_token_id: b"OUTPUT-aaaaaa",
            output_token_nonce: 1,
            price: 5u64,
            start_timestamp: 5000,
            quantity: 100,
        },
        UnmanagedAuction {
            input_token_id: b"ICE-ffffff",
            input_token_nonce: 1,
            output_token_id: b"PENG-ffffff",
            output_token_nonce: 564,
            price: 100u64,
            start_timestamp: 150000,
            quantity: 200,
        },
    ];

    for auction in expected_auctions.iter() {
        setup.create_auction_buyable_in_esdt(
            auction.input_token_id,
            auction.input_token_nonce,
            auction.output_token_id,
            auction.output_token_nonce,
            auction.price,
            auction.start_timestamp,
            auction.quantity,
        )
    }

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let actual_auctions = sc.get_all_auctions_stats();

            assert_eq!(actual_auctions.len(), expected_auctions.len());

            for index in 0..actual_auctions.len() {
                let actual_auction_stats = actual_auctions.get(index);
                let expected_auction = &expected_auctions[index];

                let expected_auction_stats = AuctionStats {
                    auction: Auction {
                        input_token_id: managed_token_id_wrapped!(expected_auction.input_token_id),
                        input_token_nonce: expected_auction.input_token_nonce,
                        output_token_id: managed_token_id!(expected_auction.output_token_id),
                        output_token_nonce: expected_auction.output_token_nonce,
                        max_quantity: managed_biguint!(expected_auction.quantity),
                        price: managed_biguint!(expected_auction.price),
                        start_timestamp: expected_auction.start_timestamp,
                        current_quantity: managed_biguint!(expected_auction.quantity),
                    },
                    id: index as u64 + 1,
                };

                assert_eq!(actual_auction_stats, expected_auction_stats);
            }
        })
        .assert_ok();
}

#[test]
fn current_quantity_updated_after_buy() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const MINT_QUANTITY: u64 = 5;
    const BUY_QUANTITY: u64 = 2;

    setup.create_default_auction_buyable_in_egld(PRICE, 0, MINT_QUANTITY);
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
                sc.buy(STARTING_AUCTION_ID);

                let auction = sc.get_auction(STARTING_AUCTION_ID);

                assert_eq!(auction.current_quantity, MINT_QUANTITY - BUY_QUANTITY);
            },
        )
        .assert_ok();
}

#[test]
fn create_auction_should_not_reanimate() {
    let mut setup = crate::helpers::setup_contract(apc_sales::contract_obj);

    const PRICE: u64 = 50;
    const QUANTITY: u64 = 30;
    const START_TIMESTAMP: u64 = 10;

    // 1. create auction A
    setup.create_default_auction_buyable_in_egld(PRICE, START_TIMESTAMP, QUANTITY);

    // 2. retireTokenFromAuction A
    setup.retire_auction(STARTING_AUCTION_ID, QUANTITY);

    // 3. create auction B with same output token
    setup.create_default_auction_buyable_in_egld(PRICE, START_TIMESTAMP, QUANTITY);

    // 4. assert that auction A has 0 remaining token
    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let first_auction_stats = sc.get_auction_stats(STARTING_AUCTION_ID);
            let second_auction_stats = sc.get_auction_stats(STARTING_AUCTION_ID + 1);

            assert_eq!(first_auction_stats.auction.current_quantity, 0);
            assert_eq!(second_auction_stats.auction.current_quantity, QUANTITY);
        })
        .assert_ok();
}
