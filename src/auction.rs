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
    pub input_token_id: TokenIdentifier<M>,
    pub input_token_nonce: u64,
    pub output_token_id: EgldOrEsdtTokenIdentifier<M>,
    pub output_token_nonce: u64,

    /** aka input amount for one output */
    pub price: BigUint<M>,

    pub start_timestamp: u64,
}
