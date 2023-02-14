#![no_std]
#![no_main]

use auction::{Auction, AuctionStats};

multiversx_sc::imports!();

pub mod auction;

pub const STARTING_AUCTION_ID: u64 = 1;

pub const ERR_SALE_IS_NOT_OPENED_YET: &str = "The sale is not opened yet";
pub const ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH: &str =
    "The payment is invalid. Wrong token sent.";
pub const ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH: &str =
    "The payment is invalid. Wrong nonce sent.";
pub const ERR_INVALID_PAYMENT_TOKEN_AMOUNT_MISMATCH: &str =
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
    #[endpoint(addTokenToAuction)]
    #[payable("*")]
    fn add_token_to_auction(&self, auction_id: u64) {
        let mut auction = self.get_auction(auction_id);
        let payment = self.call_value().single_esdt();

        require!(
            &payment.token_identifier == &auction.output_token_id,
            ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH
        );

        require!(
            &payment.token_nonce == &auction.output_token_nonce,
            ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH
        );

        auction.max_quantity += payment.amount;

        self.auctions(auction_id).set(auction);
    }

    #[only_owner]
    #[endpoint(retireTokenFromAuction)]
    fn retire_token_from_auction(&self, auction_id: u64, amount: &BigUint<Self::Api>) {
        let caller = self.blockchain().get_caller();

        let auction = self.get_auction(auction_id);

        self.send().direct_esdt(
            &caller,
            &auction.output_token_id,
            auction.output_token_nonce,
            amount,
        );
    }

    #[only_owner]
    #[endpoint(withdrawBalance)]
    fn withdraw_balance(&self) {
        let caller = self.blockchain().get_caller();

        // 1. send all EGLD balance
        self.send().direct_egld(
            &caller,
            &self
                .blockchain()
                .get_balance(&self.blockchain().get_sc_address()),
        );

        // 2. send ESDTs
        for auction_id in 1..self.next_auction_id().get() {
            let auction = self.get_auction(auction_id);

            if auction.input_token_id.is_egld() {
                // skipping egld because we already send it
                continue;
            }

            let input_token_id_esdt = auction.input_token_id.unwrap_esdt();

            let amount = self.blockchain().get_esdt_balance(
                &self.blockchain().get_sc_address(),
                &input_token_id_esdt,
                auction.input_token_nonce,
            );

            self.send().direct_esdt(
                &caller,
                &input_token_id_esdt,
                auction.input_token_nonce,
                &amount,
            );
        }
    }

    #[payable("*")]
    #[endpoint]
    fn buy(&self, auction_id: u64) {
        let auction = self.get_auction(auction_id);

        require!(
            self.blockchain().get_block_timestamp() >= auction.start_timestamp,
            ERR_SALE_IS_NOT_OPENED_YET
        );

        let payment = self.call_value().egld_or_single_esdt();

        require!(
            payment.token_identifier == auction.input_token_id,
            ERR_INVALID_PAYMENT_TOKEN_IDENTIFIER_MISMATCH
        );

        require!(
            payment.token_nonce == auction.input_token_nonce,
            ERR_INVALID_PAYMENT_TOKEN_NONCE_MISMATCH
        );

        require!(
            &payment.amount > &0 && &payment.amount % &auction.price == 0,
            ERR_INVALID_PAYMENT_TOKEN_AMOUNT_MISMATCH
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

    fn get_auction(&self, auction_id: u64) -> Auction<Self::Api> {
        require!(
            !self.auctions(auction_id).is_empty(),
            ERR_INVALID_AUCTION_ID
        );

        return self.auctions(auction_id).get();
    }

    #[view(getAuctionStats)]
    fn get_auction_stats(&self, auction_id: u64) -> AuctionStats<Self::Api> {
        let auction = self.get_auction(auction_id);

        let remaining_output_items = self.get_remaining_amount(&auction);

        return AuctionStats {
            auction,
            remaining_output_items,
            id: auction_id,
        };
    }

    #[view(getAllAuctionStats)]
    fn get_all_auctions_stats(&self) -> ManagedVec<Self::Api, AuctionStats<Self::Api>> {
        let mut all_auctions = ManagedVec::new();

        for auction_id in STARTING_AUCTION_ID..self.next_auction_id().get() {
            let auction = self.get_auction_stats(auction_id);

            all_auctions.push(auction);
        }

        return all_auctions;
    }

    fn get_remaining_amount(&self, auction: &Auction<Self::Api>) -> BigUint<Self::Api> {
        self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            &auction.output_token_id,
            auction.output_token_nonce,
        )
    }
}
