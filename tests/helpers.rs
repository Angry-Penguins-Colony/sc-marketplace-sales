use apc_sales::*;
use multiversx_sc::types::{Address, BoxedBytes, EgldOrEsdtTokenIdentifier};
use multiversx_sc_scenario::{
    managed_biguint, managed_token_id_wrapped, rust_biguint, testing_framework::*, DebugApi,
};

const WASM_PATH: &str = "output/apc_sales.wasm";

pub const DEFAULT_AUCTION_OUTPUT_TOKEN: &[u8] = b"SELL-aaaaaa";
pub const DEFAULT_AUCTION_OUTPUT_NONCE: u64 = 1u64;

pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> apc_sales::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract_wrapper: ContractObjWrapper<apc_sales::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> apc_sales::ContractObj<DebugApi>,
{
    pub fn create_default_auction_buyable_in_egld(
        &mut self,
        price: u64,
        start_timestamp: u64,
        quantity: u64,
    ) {
        self.create_auction_buyable_in_egld(
            DEFAULT_AUCTION_OUTPUT_TOKEN,
            DEFAULT_AUCTION_OUTPUT_NONCE,
            price,
            start_timestamp,
            quantity,
        );
    }

    pub fn create_auction_buyable_in_egld(
        &mut self,
        output_token_id: &[u8],
        output_token_nonce: u64,
        price: u64,
        start_timestamp: u64,
        quantity: u64,
    ) {
        self.blockchain_wrapper.set_nft_balance(
            &self.owner_address,
            output_token_id,
            output_token_nonce,
            &rust_biguint!(quantity),
            &BoxedBytes::empty(),
        );

        self.blockchain_wrapper
            .execute_esdt_transfer(
                &self.owner_address,
                &self.contract_wrapper,
                output_token_id,
                output_token_nonce,
                &rust_biguint!(quantity),
                |sc| {
                    let _ = sc.create_auction(
                        EgldOrEsdtTokenIdentifier::egld(),
                        0,
                        managed_biguint!(price),
                        start_timestamp,
                    );
                },
            )
            .assert_ok();
    }

    pub fn create_auction_buyable_in_esdt(
        &mut self,
        input_token_id: &[u8],
        input_token_nonce: u64,
        output_token_id: &[u8],
        output_token_nonce: u64,
        price: u64,
        start_timestamp: u64,
        quantity: u64,
    ) {
        self.blockchain_wrapper.set_nft_balance(
            &self.owner_address,
            output_token_id,
            output_token_nonce,
            &rust_biguint!(quantity),
            &BoxedBytes::empty(),
        );

        self.blockchain_wrapper
            .execute_esdt_transfer(
                &self.owner_address,
                &self.contract_wrapper,
                output_token_id,
                output_token_nonce,
                &rust_biguint!(quantity),
                |sc| {
                    let _ = sc.create_auction(
                        managed_token_id_wrapped!(input_token_id),
                        input_token_nonce,
                        managed_biguint!(price),
                        start_timestamp,
                    );
                },
            )
            .assert_ok();
    }
}

pub fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> apc_sales::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        user_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test]
fn deploy_test() {
    let mut setup = setup_contract(apc_sales::contract_obj);

    // simulate deploy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
            },
        )
        .assert_ok();
}
