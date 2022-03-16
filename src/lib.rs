#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod state;
mod storage;

use crate::state::StakeType;
use crate::state::StakeNode;

#[elrond_wasm::derive::contract]
pub trait LandboardStaking: storage::StorageModule{
    #[init]
    fn init(&self, stake_token_id: TokenIdentifier, reward_token_id: TokenIdentifier) {
        require!(
            stake_token_id.is_valid_esdt_identifier(),
            "invalid stake_token_id"
        );
        self.stake_token_id().set(&stake_token_id);

        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );
        self.reward_token_id().set(&reward_token_id);
    }

    #[payable("*")]
    #[endpoint]
    fn stake(&self, #[payment_token] payment_token_id: TokenIdentifier, #[payment_amount] payment_amount: BigUint, stake_type_id: usize) {
        require!(
            payment_token_id == self.stake_token_id().get(),
            "invalid payment_token_id"
        );
        require!(
            0 < stake_type_id && stake_type_id <= self.stake_types().len(),
            "invalid stake_type_id"
        );

        let stake_type = self.stake_types().get(stake_type_id);

        require!(
            payment_amount >= stake_type.min_stake_limit,
            "cannot stake less than min_stake_limit"
        );

        let caller = self.blockchain().get_caller();

        // if caller is a new address, add it to staker_addresses
        if !self.staker_addresses().contains(&caller) {
            self.staker_addresses().insert(caller.clone());
        }

        let new_node_id = self.last_node_id(&caller).get() + 1;

        self.node_ids(&caller).insert(new_node_id);
        let stake_node = StakeNode {
            node_id: new_node_id,
            stake_type: stake_type,
            stake_amount: payment_amount,
            stake_timestamp: self.blockchain().get_block_timestamp()
        };

        self.nodes(&caller, new_node_id).set(stake_node);
    }
}