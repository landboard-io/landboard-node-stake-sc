elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeType<M: ManagedTypeApi> {
    pub stake_type_id: u32,
    pub locking_timestamp: u64,     // timestamp for locking staked tokens
    pub delegation_timestamp: u64,  // interval between unstaking and claimable state
    pub min_stake_limit: BigUint<M>,
    pub tax: u32,       // tax; 1000 = 10%
    pub apy: u32,       // apy; 5000 = 50%
    pub disabled: bool, // cannot create disabled stake_type
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeNode<M: ManagedTypeApi> {
    pub node_id: u32,

    pub stake_type_id: u32,
    pub locking_timestamp: u64,
    pub delegation_timestamp: u64,
    pub tax: u32,
    pub apy: u32,

    pub stake_amount: BigUint<M>,
    pub stake_timestamp: u64,

    // if unstaked, stake_amount and reward will be undelegated for delegation_timestamp until claimable
    pub state: u32, // 1 for not-unstakable, 2 for unstakable, 3 for unstaked and not-claimable, 4 for claimable
    pub reward_amount: BigUint<M>,
    pub unstake_timestamp: u64,
}