elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeType<M: ManagedTypeApi> {
    pub stake_type_id: usize,
    pub locking_timestamp: u64,     // timestamp for locking staked tokens
    pub delegation_timestamp: u64,  // interval between unstaking and claimable state
    pub min_stake_limit: BigUint<M>,
    pub tax: u32,       // tax; 1000 = 10%
    pub apy: u32,       // apy; 5000 = 50%
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeNode<M: ManagedTypeApi> {
    pub node_id: usize,
    pub stake_type: StakeType<M>,
    pub stake_amount: BigUint<M>,
    pub stake_timestamp: u64,

    // if unstaked, stake_amount and reward will be undelegated for delegation_timestamp until claimable
    pub unstaked: bool,
    pub reward_amount: BigUint<M>,
    pub unstake_timestamp: u64,
}