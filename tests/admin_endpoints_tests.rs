use apc_sales::{EmptyContract, STARTING_AUCTION_ID};
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

            assert_eq!(auction.remaining_output_items, REMAINING_QUANTITY);
        })
        .assert_ok();
}
