#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[payable("*")]
    #[endpoint(createAuction)]
    fn create_auction(
        &self,
        token: TokenIdentifier,
        nonce: u64,
        price: BigUint,
    ) {
        // let payments = self.call_value().all_esdt_transfers();
        todo!();
    }

    #[only_owner]
    #[endpoint(stopAuction)]
    fn stop_auction(
        &self, 
        id: u64
    ) {
        todo!();
    }

    #[only_owner]
    #[endpoint(retireTokenFromAuction)]
    fn retire_token_from_auction(
        &self,
        id: u64, 
        amount: u64
    ) {
        todo!();
    }

    #[only_owner]
    #[endpoint(withdrawBalance)]
    fn withdraw_balance(&self) {
        todo!();
    }

    #[payable("*")]
    #[endpoint]
    fn buy(
        &self,
        quantity: u64
    ) {
        todo!();
    }

    #[view(getAuction)]
    fn get_auction(
        &self,
        id: u64
    ) {
        todo!();
    }

    #[view(getActiveAuctions)]
    fn get_active_auctions(&self) {
        todo!();
    }
}
