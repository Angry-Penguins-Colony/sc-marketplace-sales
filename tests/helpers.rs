use apc_sales::*;
use multiversx_sc::types::{Address, BoxedBytes};
use multiversx_sc_scenario::{
    managed_biguint, managed_token_id_wrapped, rust_biguint, testing_framework::*, DebugApi,
};

const WASM_PATH: &str = "output/apc_sales.wasm";

pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> apc_sales::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper: ContractObjWrapper<apc_sales::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> apc_sales::ContractObj<DebugApi>,
{
    pub fn create_auction(
        &mut self,
        sell_token: &[u8],
        sell_nonce: u64,
        price_token_identifier: &[u8],
        price_token_nonce: u64,
        price: u64,
        start_timestamp: u64,
        quantity: u64,
    ) {
        self.blockchain_wrapper.set_nft_balance(
            &self.owner_address,
            sell_token,
            sell_nonce,
            &rust_biguint!(quantity),
            &BoxedBytes::empty(),
        );

        self.blockchain_wrapper
            .execute_esdt_transfer(
                &self.owner_address,
                &self.contract_wrapper,
                sell_token,
                sell_nonce,
                &rust_biguint!(1),
                |sc| {
                    let _ = sc.create_auction(
                        managed_token_id_wrapped!(price_token_identifier),
                        price_token_nonce,
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
