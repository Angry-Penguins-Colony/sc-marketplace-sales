multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    PartialEq,
    TypeAbi,
    Clone,
    Debug,
)]
pub struct Auction<M: ManagedTypeApi> {
    pub sell_token: TokenIdentifier<M>,
    pub sell_nonce: u64,
    pub price_token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub price_token_nonce: u64,
    pub price: BigUint<M>,
    pub start_timestamp: u64,
}
