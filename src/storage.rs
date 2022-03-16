elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{StakeType, StakeNode};


const MAX_TAX: u32 = 1000; // 10%

#[elrond_wasm::module]
pub trait StorageModule {
    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        self.paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        self.paused().set(false);
    }

    #[only_owner]
    #[endpoint(setStakeTypes)]
    fn set_stake_types(&self, #[var_args] stake_types: MultiValueEncoded<MultiValue3<u64, BigUint, u32>>) {
        for item in stake_types.into_iter() {
            let (locking_timestamp, min_stake_limit, tax) = item.into_tuple();

            require!(
                tax <= MAX_TAX,
                "tax cannot be greater than MAX_TAX 1000"
            );

            let new_stake_type = StakeType {
                locking_timestamp,
                min_stake_limit,
                tax
            };
            self.stake_types().push(&new_stake_type);
        }
    }

    #[only_owner]
    #[endpoint(clearStakeTypes)]
    fn clear_stake_types(&self) {
        self.stake_types().clear();
    }

    ///////////////////////////////////////////////////////////
    
    #[view(getPaused)]
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;

    //

    #[view(getStakeTypes)]
    #[storage_mapper("stake_types")]
    fn stake_types(&self) -> VecMapper<StakeType<Self::Api>>;

    #[view(getStakerAddresses)]
    #[storage_mapper("staker_addresses")]
    fn staker_addresses(&self) -> SetMapper<ManagedAddress>;

    #[view(getNodeIds)]
    #[storage_mapper("node_ids")]
    fn node_ids(&self, staker_address: &ManagedAddress) -> SetMapper<u32>;

    #[view(getNode)]
    #[storage_mapper("nodes")]
    fn nodes(&self, staker_address: &ManagedAddress, node_id: u32) -> SingleValueMapper<StakeNode<Self::Api>>;
}