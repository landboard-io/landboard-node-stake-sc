elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait EventModule {
    #[event("stake")]
    fn stake_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] node_id: usize,
        #[indexed] stake_type_id: usize,
        #[indexed] stake_amount: BigUint,
        #[indexed] stake_timestamp: u64,
    );

    #[event("referral")]
    fn referral_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] referrer: ManagedAddress,
    );

    #[event("referral_activated")]
    fn referral_activated_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] referrer: ManagedAddress,
        #[indexed] timestamp: u64,
        #[indexed] amount: BigUint,
    );

    #[event("unstake")]
    fn unstake_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] node_id: usize,
        #[indexed] stake_amount: BigUint,
        #[indexed] stake_timestamp: u64,
        #[indexed] reward_amount: BigUint,
        #[indexed] unstake_timestamp: u64,
    );

    #[event("claim")]
    fn claim_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] node_id: usize,
        #[indexed] stake_amount: BigUint,
        #[indexed] stake_timestamp: u64,
        #[indexed] reward_amount: BigUint,
        #[indexed] claim_timestamp: u64,
    );
}