#![no_std]
#![no_main]

use auction::Auction;

multiversx_sc::imports!();

pub mod auction;

pub const STARTING_AUCTION_ID: u64 = 1;

pub const ERR_SALE_IS_NOT_OPENED_YET: &str = "The sale is not opened yet";
pub const ERR_INVALID_PAYMENT_WRONG_TOKEN_SENT: &str = "The payment is invalid. Wrong token sent.";
pub const ERR_INVALID_PAYMENT_WRONG_NONCE_SENT: &str = "The payment is invalid. Wrong nonce sent.";
pub const ERR_INVALID_PAYMENT_WRONG_AMOUNT_SENT: &str =
    "The payment is invalid. Wrong amount sent.";
pub const ERR_NOT_ENOUGHT_ITEMS: &str = "Cannot fulfill your order. Try to buy less items.";
pub const ERR_BUYING_UNEXISTING_AUCTION: &str = "Auction ID invalid.";

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
    fn buy(&self, _id: u64, _quantity: u64) {
        require!(
            !self.auctions(_id).is_empty(),
            ERR_BUYING_UNEXISTING_AUCTION
        );

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
