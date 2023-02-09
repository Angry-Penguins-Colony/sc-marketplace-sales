#![no_std]
#![no_main]

use auction::Auction;

multiversx_sc::imports!();

pub mod auction;

const STARTING_AUCTION_ID: u64 = 1;

#[multiversx_sc::contract]
pub trait EmptyContract {
    #[storage_mapper("auctions")]
    fn auctions(&self, id: u64) -> SingleValueMapper<Auction<Self::Api>>;

    #[storage_mapper("next_auction_id")]
    fn next_auction_id(&self) -> SingleValueMapper<u64>;

    #[init]
    fn init(&self) {
        self.next_auction_id().set(STARTING_AUCTION_ID);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(createAuction)]
    fn create_auction(
        &self,
        price_token_identifier: EgldOrEsdtTokenIdentifier,
        price_token_nonce: u64,
        price: BigUint,
        start_timestamp: u64,
    ) -> u64 {
        let payments = self.call_value().single_esdt();

        let new_auction_id = self.next_auction_id().get();
        self.auctions(new_auction_id).set(Auction {
            price,
            price_token_nonce,
            price_token_identifier,
            start_timestamp,
            sell_nonce: payments.token_nonce,
            sell_token: payments.token_identifier,
        });

        self.next_auction_id().set(new_auction_id + 1);

        return new_auction_id;
    }

    #[only_owner]
    #[endpoint(stopAuction)]
    fn stop_auction(&self, _id: u64) {
        todo!();
    }

    #[only_owner]
    #[endpoint(retireTokenFromAuction)]
    fn retire_token_from_auction(&self, _id: u64, _amount: u64) {
        todo!();
    }

    #[only_owner]
    #[endpoint(withdrawBalance)]
    fn withdraw_balance(&self) {
        todo!();
    }

    #[payable("*")]
    #[endpoint]
    fn buy(&self, _quantity: u64) {
        todo!();
    }

    #[view(getAuction)]
    fn get_auction(&self, _id: u64) {
        todo!();
    }

    #[view(getActiveAuctions)]
    fn get_active_auctions(&self) {
        todo!();
    }
}
