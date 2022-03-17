#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod state;
mod storage;

use crate::state::StakeNode;

const TOTAL_PERCENTAGE: u32 = 10000; // 100%

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
    fn stake(&self, #[payment_token] payment_token_id: TokenIdentifier, #[payment_amount] payment_amount: BigUint, stake_type_id: usize, #[var_args] opt_referrer_address: OptionalValue<ManagedAddress>) {
        self.require_activation();

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

            if let OptionalValue::Some(v) = opt_referrer_address {
                self.referrer_address(&caller).set(&v);
            }
        }

        if payment_amount >= self.referral_activation_amount().get() && !self.referral_activated(&caller).get() {
            let referrer_address = &self.referrer_address(&caller).get();
            let new_referred_count = self.referred_count(&referrer_address).get() + 1;

            if new_referred_count <= self.max_apy_increase_by_referral().get() {
                self.referred_count(&referrer_address).set(new_referred_count);
            }
            self.referral_activated(&caller).set(true);
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
        self.last_node_id(&caller).set(&new_node_id);
    }

    #[endpoint]
    fn unstake(&self, node_id: usize) {
        self.require_activation();

        let caller = self.blockchain().get_caller();

        require!(
            self.node_ids(&caller).contains(&node_id),
            "node_id does not exist"
        );

        let stake_node = self.nodes(&caller, node_id).get();
        let (_, reward_amount) = self.get_claimable_and_reward(&stake_node);

        let stake_amount = stake_node.stake_amount.clone();

        require!(
            stake_amount <= self.blockchain().get_sc_balance(&self.stake_token_id().get(), 0),
            "not enough stake tokens in smart contract"
        );
        require!(
            reward_amount <= self.blockchain().get_sc_balance(&self.reward_token_id().get(), 0),
            "not enough reward tokens in smart contract"
        );

        // clear old storage
        self.node_ids(&caller).remove(&node_id);
        self.nodes(&caller, node_id).clear();
        if self.node_ids(&caller).is_empty() {
            self.staker_addresses().remove(&caller);
        }

        self.send().direct(&caller, &self.stake_token_id().get(), 0, &stake_amount, b"return staked tokens");
        self.send().direct(&caller, &self.stake_token_id().get(), 0, &reward_amount, b"return reward tokens");
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self,
        #[var_args] opt_token_id: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_amount: OptionalValue<BigUint>) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };
        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, 0, &token_amount, &[]);
    }


    /// private

    #[inline]
    fn get_claimable_and_reward(&self, stake_node: &StakeNode<Self::Api>) -> (bool, BigUint) {
        let caller = self.blockchain().get_caller();
        let stake_type = &stake_node.stake_type;

    
        let roi = stake_type.roi + self.referred_count(&caller).get() * self.apy_increase_per_referral().get();
        let mut reward_amount = self.calculate_reward(stake_node.stake_amount.clone(), roi);

        // if it's before locking_timestamp, charge tax to reward
        if self.blockchain().get_block_timestamp() < stake_node.stake_timestamp + stake_type.locking_timestamp {
            reward_amount = reward_amount * &BigUint::from(self.blockchain().get_block_timestamp() - stake_node.stake_timestamp) / &BigUint::from(stake_type.locking_timestamp) * &BigUint::from(stake_type.tax) / &BigUint::from(TOTAL_PERCENTAGE);

            return (false, reward_amount);
        }

        (true, reward_amount)
    }

    #[inline]
    fn calculate_reward(&self, base_amount: BigUint, roi: u32) -> BigUint {
        base_amount * &BigUint::from(roi) / &BigUint::from(TOTAL_PERCENTAGE)
    }

    fn require_activation(&self) {
        require!(
            !self.paused().get(),
            "staking is paused"
        );
    }

    /// view
    
    #[view(getNodesPerStaker)]
    fn get_nodes_per_staker(&self, staker_address: ManagedAddress) -> MultiValueEncoded<MultiValue6<usize, BigUint, u64, u64, bool, BigUint>> {
        let mut items_vec = MultiValueEncoded::new();
        for node_id in self.node_ids(&staker_address).iter() {
            let stake_node = self.nodes(&staker_address, node_id).get();

            let (claimable, reward_amount) = self.get_claimable_and_reward(&stake_node);

            items_vec.push(
                MultiValue6::from((
                    stake_node.node_id,
                    stake_node.stake_amount,
                    stake_node.stake_timestamp,
                    stake_node.stake_type.locking_timestamp,
                    claimable,
                    reward_amount
                ))
            );
        }

        items_vec
    }
}