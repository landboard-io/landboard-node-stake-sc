elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeType<M: ManagedTypeApi> {
    pub locking_timestamp: u64,
    pub min_stake_limit: BigUint<M>,
    pub tax: u32
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeNode<M: ManagedTypeApi> {
    pub stake_type: StakeType<M>,
    pub stake_amount: BigUint<M>,
    pub stake_timestamp: u64
}