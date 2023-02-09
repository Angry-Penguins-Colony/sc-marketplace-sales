#![no_std]
#![no_main]

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
        _token: TokenIdentifier,
        _nonce: u64,
        _price: BigUint,
    ) {
        // let payments = self.call_value().all_esdt_transfers();
        todo!();
    }

    #[only_owner]
    #[endpoint(stopAuction)]
    fn stop_auction(
        &self, 
        _id: u64
    ) {
        todo!();
    }

    #[only_owner]
    #[endpoint(retireTokenFromAuction)]
    fn retire_token_from_auction(
        &self,
        _id: u64, 
        _amount: u64
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
        _quantity: u64
    ) {
        todo!();
    }

    #[view(getAuction)]
    fn get_auction(
        &self,
        _id: u64
    ) {
        todo!();
    }

    #[view(getActiveAuctions)]
    fn get_active_auctions(&self) {
        todo!();
    }
}
