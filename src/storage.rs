elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{StakeType, StakeNode};


const MAX_TAX: u32 = 10000; // 100%

#[elrond_wasm::module]
pub trait StorageModule {
    #[only_owner]
    #[endpoint(setStakeTokenId)]
    fn set_stake_token_id(&self, stake_token_id: TokenIdentifier) {
        require!(
            stake_token_id.is_valid_esdt_identifier(),
            "invalid stake_token_id"
        );
        self.stake_token_id().set(&stake_token_id);
    }

    #[only_owner]
    #[endpoint(setRewardTokenId)]
    fn set_reward_token_id(&self, reward_token_id: TokenIdentifier) {
        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );
        self.reward_token_id().set(&reward_token_id);
    }

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
    #[endpoint(addStakeTypes)]
    fn add_stake_types(&self, #[var_args] stake_types: MultiValueEncoded<MultiValue4<u64, BigUint, u32, u32>>) {
        for item in stake_types.into_iter() {
            let (locking_timestamp, min_stake_limit, tax, roi) = item.into_tuple();

            require!(
                tax <= MAX_TAX,
                "tax cannot be greater than MAX_TAX 1000"
            );

            let new_stake_type = StakeType {
                locking_timestamp,
                min_stake_limit,
                tax,
                roi
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
    
    #[view(getStakeTokenId)]
    #[storage_mapper("stake_token_id")]
    fn stake_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

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
    fn node_ids(&self, staker_address: &ManagedAddress) -> SetMapper<usize>;

    #[view(getLastNodeId)]
    #[storage_mapper("last_node_id")]
    fn last_node_id(&self, staker_address: &ManagedAddress) -> SingleValueMapper<usize>;

    #[view(getNode)]
    #[storage_mapper("nodes")]
    fn nodes(&self, staker_address: &ManagedAddress, node_id: usize) -> SingleValueMapper<StakeNode<Self::Api>>;
}