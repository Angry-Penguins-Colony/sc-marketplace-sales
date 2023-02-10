#![no_std]
#![no_main]

use auction::{Auction, AuctionStats};

multiversx_sc::imports!();

pub mod auction;

pub const STARTING_AUCTION_ID: u64 = 1;

pub const ERR_SALE_IS_NOT_OPENED_YET: &str = "The sale is not opened yet";
pub const ERR_INVALID_PAYMENT_WRONG_TOKEN_SENT: &str = "The payment is invalid. Wrong token sent.";
pub const ERR_INVALID_PAYMENT_WRONG_NONCE_SENT: &str = "The payment is invalid. Wrong nonce sent.";
pub const ERR_INVALID_PAYMENT_WRONG_AMOUNT_SENT: &str =
    "The payment is invalid. Wrong amount sent.";
pub const ERR_NOT_ENOUGHT_ITEMS: &str = "Cannot fulfill your order. Try to buy less items.";
pub const ERR_INVALID_AUCTION_ID: &str = "Auction ID invalid.";
pub const ERR_CREATE_AUCTION_BAD_EGLD_NONCE: &str =
    "When creating an auction with egld, you must set the nonce to 0.";
pub const ERR_CREATE_AUCTION_BAD_PRICE: &str = "The price cannot be set to 0";

#[multiversx_sc::contract]
pub trait EmptyContract {
    #[storage_mapper("auctions")]
    fn auctions(&self, id: u64) -> SingleValueMapper<Auction<Self::Api>>;

    #[storage_mapper("next_auction_id")]
    #[view(getNextAuctionId)]
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
        input_token_id: EgldOrEsdtTokenIdentifier,
        input_token_nonce: u64,
        price: BigUint,
        start_timestamp: u64,
    ) -> u64 {
        if input_token_id.is_egld() {
            require!(input_token_nonce == 0, ERR_CREATE_AUCTION_BAD_EGLD_NONCE);
        }

        require!(price > 0, ERR_CREATE_AUCTION_BAD_PRICE);

        let payment = self.call_value().single_esdt();

        let new_auction_id = self.next_auction_id().get();
        self.auctions(new_auction_id).set(Auction {
            price,
            start_timestamp,
            input_token_id,
            input_token_nonce,
            output_token_nonce: payment.token_nonce,
            output_token_id: payment.token_identifier,
            max_quantity: payment.amount,
        });

        self.next_auction_id().set(new_auction_id + 1);

        return new_auction_id;
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
    fn buy(&self, auction_id: u64) {
        self.require_valid_auction_id(auction_id);

        let auction = self.auctions(auction_id).get();

        require!(
            self.blockchain().get_block_timestamp() >= auction.start_timestamp,
            ERR_SALE_IS_NOT_OPENED_YET
        );

        let payment = self.call_value().egld_or_single_esdt();

        require!(
            payment.token_identifier == auction.input_token_id,
            ERR_INVALID_PAYMENT_WRONG_TOKEN_SENT
        );

        require!(
            payment.token_nonce == auction.input_token_nonce,
            ERR_INVALID_PAYMENT_WRONG_NONCE_SENT
        );

        require!(
            &payment.amount > &0 && &payment.amount % &auction.price == 0,
            ERR_INVALID_PAYMENT_WRONG_AMOUNT_SENT
        );

        let wanted_buy_amount = payment.amount / &auction.price;

        require!(
            self.get_remaining_amount(&auction) >= wanted_buy_amount,
            ERR_NOT_ENOUGHT_ITEMS
        );

        // Send nfts
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &auction.output_token_id,
            auction.output_token_nonce,
            &wanted_buy_amount,
        );
    }

    #[view(getAuction)]
    fn get_auction(&self, auction_id: u64) -> AuctionStats<Self::Api> {
        self.require_valid_auction_id(auction_id);

        let auction = self.auctions(auction_id).get();
        let remaining_output_items = self.get_remaining_amount(&auction);

        return AuctionStats {
            auction,
            remaining_output_items,
        };
    }

    fn require_valid_auction_id(&self, auction_id: u64) {
        require!(
            !self.auctions(auction_id).is_empty(),
            ERR_INVALID_AUCTION_ID
        );
    }

    fn get_remaining_amount(&self, auction: &Auction<Self::Api>) -> BigUint<Self::Api> {
        self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            &auction.output_token_id,
            auction.output_token_nonce,
        )
    }
}
