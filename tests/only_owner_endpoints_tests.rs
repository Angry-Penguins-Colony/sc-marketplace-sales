use apc_sales::EndpointWrappers;
use multiversx_sc_scenario::rust_biguint;

use crate::helpers;

const ONLY_OWNER_ERR_MESSAGE: &str = "Endpoint can only be called by owner";

#[test]
fn create_auction_is_forbidden() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_create_auction();
            },
        )
        .assert_user_error(ONLY_OWNER_ERR_MESSAGE);
}

#[test]
fn add_token_to_auction_is_forbidden() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_add_token_to_auction();
            },
        )
        .assert_user_error(ONLY_OWNER_ERR_MESSAGE);
}

#[test]
fn retire_token_from_auction_is_forbidden() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_retire_token_from_auction();
            },
        )
        .assert_user_error(ONLY_OWNER_ERR_MESSAGE);
}

#[test]
fn withdraw_balance() {
    let mut setup = helpers::setup_contract(apc_sales::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_withdraw_balance();
            },
        )
        .assert_user_error(ONLY_OWNER_ERR_MESSAGE);
}
