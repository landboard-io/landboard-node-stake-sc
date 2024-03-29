#![no_std]
#![feature(generic_associated_types)]
#![feature(let_chains)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod state;
mod storage;
mod event;

use crate::state::StakeNode;

const TOTAL_PERCENTAGE: u32 = 10000; // 100%
const DAY_IN_SECONDS: u64 = 3600 * 24;
// const DAY_IN_SECONDS: u64 = 60;
const YEAR_IN_DAYS: u64 = 365;

#[elrond_wasm::derive::contract]
pub trait LandboardStaking:
    storage::StorageModule
    + event::EventModule
{
    #[init]
    fn init(
        &self,
        stake_token_id: TokenIdentifier,
        reward_token_id: TokenIdentifier,
        referral_activation_amount: BigUint,
        apy_increase_per_referral: u32,
        max_apy_increase_by_referral: u32,
        promo_increase_apy: u32,
        total_referral_count_limit: u32,
    ) {
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

        self.referral_activation_amount().set(&referral_activation_amount);

        require!(
            apy_increase_per_referral <= TOTAL_PERCENTAGE,
            "cannot be greater than 10000"
        );
        self.apy_increase_per_referral().set(apy_increase_per_referral);

        require!(
            max_apy_increase_by_referral <= TOTAL_PERCENTAGE,
            "cannot be greater than 10000"
        );
        self.max_apy_increase_by_referral().set(max_apy_increase_by_referral);
        self.promo_increase_apy().set(promo_increase_apy);
        self.total_referral_count_limit().set(total_referral_count_limit);
    }

    /*
        @notice if caller and referrer_address are same, it means there is no referrer
     */
    #[payable("*")]
    #[endpoint]
    fn stake(
        &self,
        #[payment_token] payment_token_id: TokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
        stake_type_id: u32,
        #[var_args] opt_referrer_address: OptionalValue<ManagedAddress>
    ) {
        self.require_activation();

        require!(
            payment_token_id == self.stake_token_id().get(),
            "invalid payment_token_id"
        );
        require!(
            0 < stake_type_id && stake_type_id <= self.stake_types().len() as u32,
            "invalid stake_type_id"
        );

        let stake_type = self.stake_types().get(stake_type_id as usize);

        require!(
            !stake_type.disabled,
            "this stake_type is disabled"
        );
        require!(
            payment_amount >= stake_type.min_stake_limit,
            "cannot stake less than min_stake_limit"
        );

        let caller = self.blockchain().get_caller();

        // if caller is a new address, add it to staker_addresses
        if !self.staker_addresses().contains(&caller) {
            self.staker_addresses().insert(caller.clone());

            // if referrer_address is given and referrer_address mapper is empty, store it to referrer_address mapper
            if let OptionalValue::Some(referrer_address) = opt_referrer_address && self.referrer_address(&caller).is_empty() {
                require!(
                    caller != referrer_address,
                    "referrer cannot be yourself"
                );

                self.referrer_address(&caller).set(&referrer_address);

                self.referral_event(caller.clone(), referrer_address);
            }
        }

        // activate referral if the caller stakes more than referral_activation_amount and referral is not activated yet
        if payment_amount >= self.referral_activation_amount().get()
            && !self.referral_activated(&caller).get()
            && self.total_referral_count().get() < self.total_referral_count_limit().get() 
            && !self.referrer_address(&caller).is_empty()
        {
            let referrer_address = &self.referrer_address(&caller).get();
            let new_referred_count = self.referred_count(&referrer_address).get() + 1;

            if new_referred_count * self.apy_increase_per_referral().get() <= self.max_apy_increase_by_referral().get() {
                self.referred_count(&referrer_address).set(new_referred_count);
            }
            self.referral_activated(&caller).set(true);
            self.total_referral_count().update(|v| *v += 1u32);

            self.referral_activated_event(caller.clone(), referrer_address.clone(), self.blockchain().get_block_timestamp(), payment_amount.clone());
        }

        let new_node_id = self.last_node_id(&caller).get() + 1;

        self.node_ids(&caller).insert(new_node_id);
        let stake_node = StakeNode {
            node_id: new_node_id,
            
            stake_type_id: stake_type.stake_type_id,
            locking_timestamp: stake_type.locking_timestamp,
            delegation_timestamp: stake_type.delegation_timestamp,
            tax: stake_type.tax,
            apy: stake_type.apy,

            stake_amount: payment_amount.clone(),
            stake_timestamp: self.blockchain().get_block_timestamp(),
            
            state: 1,    // not-unstakable
            reward_amount: BigUint::zero(),
            unstake_timestamp: 0u64,
        };

        self.nodes(&caller, new_node_id).set(stake_node);
        self.last_node_id(&caller).set(&new_node_id);

        self.stake_event(caller, new_node_id, stake_type_id, payment_amount, self.blockchain().get_block_timestamp());
    }

    /*
        @dev if caller unstakes node before locking, reward will be reduced by tax and timstamp ratio
        unstaked node will be delegated by delegated_timestamp
    */
    #[endpoint]
    fn unstake(
        &self,
        node_id: u32
    ) {
        self.require_activation();

        let caller = self.blockchain().get_caller();

        require!(
            self.node_ids(&caller).contains(&node_id),
            "node_id does not exist"
        );

        let mut stake_node = self.nodes(&caller, node_id).get();

        require!(
            stake_node.state < 3,
            "node was already unstaked"
        );

        let (_, reward_amount) = self.get_claimable_and_reward(&caller, node_id);

        stake_node.state = 3;
        stake_node.reward_amount = reward_amount.clone();
        stake_node.unstake_timestamp = self.blockchain().get_block_timestamp();

        // if locking_timestamp is passed, delegation_timestamp will be 0 and node can be claimed anytime
        if stake_node.stake_timestamp + stake_node.locking_timestamp <= stake_node.unstake_timestamp {
            stake_node.delegation_timestamp = 0;
        }
        
        self.unstake_event(caller.clone(), node_id, stake_node.stake_amount.clone(), stake_node.stake_timestamp, reward_amount, self.blockchain().get_block_timestamp());

        self.nodes(&caller, node_id).set(stake_node);
    }

    /*
        @dev unstaked node can be claimed after delegated_timestamp
    */
    #[endpoint]
    fn claim(
        &self,
        node_id: u32
    ) {
        self.require_activation();

        let caller = self.blockchain().get_caller();

        require!(
            self.node_ids(&caller).contains(&node_id),
            "node_id does not exist"
        );

        let stake_node = self.nodes(&caller, node_id).get();

        require!(
            stake_node.state >= 3,
            "not claimable - node is not unstaked"
        );
        require!(
            self.blockchain().get_block_timestamp() >= stake_node.unstake_timestamp + stake_node.delegation_timestamp,
            "cannot claim before delegation_timestamp"
        );

        let stake_amount = stake_node.stake_amount.clone();
        let reward_amount = stake_node.reward_amount.clone();
        
        // check balance of SC whether have enought tokens to send
        if self.stake_token_id().get() == self.reward_token_id().get() {
            require!(
                stake_amount.clone() + &reward_amount <= self.blockchain().get_sc_balance(&self.stake_token_id().get(), 0),
                "not enough stake/reward tokens in smart contract"
            );
        } else {
            require!(
                stake_amount <= self.blockchain().get_sc_balance(&self.stake_token_id().get(), 0),
                "not enough stake tokens in smart contract"
            );
            require!(
                reward_amount <= self.blockchain().get_sc_balance(&self.reward_token_id().get(), 0),
                "not enough reward tokens in smart contract"
            );
        }

        // clear old storage
        self.node_ids(&caller).remove(&node_id);
        self.nodes(&caller, node_id).clear();
        if self.node_ids(&caller).is_empty() {
            self.staker_addresses().remove(&caller);
        }

        if stake_amount > BigUint::zero() {
            self.send().direct(&caller, &self.stake_token_id().get(), 0, &stake_amount, b"return staked tokens");
        }

        if reward_amount > BigUint::zero() {
            self.send().direct(&caller, &self.reward_token_id().get(), 0, &reward_amount, b"return reward tokens");
        }

        self.claim_event(caller, node_id, stake_amount, stake_node.stake_timestamp, reward_amount, self.blockchain().get_block_timestamp());
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self,
        #[var_args] opt_token_id: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_amount: OptionalValue<BigUint>
    ) {
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

    #[view(getClaimableAndReward)]
    fn get_claimable_and_reward(
        &self,
        caller: &ManagedAddress,
        stake_node_id: u32,
    ) -> (bool, BigUint) {
        let stake_node = self.nodes(caller, stake_node_id).get();
        let apy = self.get_apy_of_staker(caller, stake_node.node_id);

        let mut reward_amount = self.calculate_reward(stake_node.stake_amount.clone(), stake_node.locking_timestamp, apy);

        // if it's before locking_timestamp, charge tax to reward
        if self.blockchain().get_block_timestamp() < stake_node.stake_timestamp + stake_node.locking_timestamp {
            reward_amount = reward_amount * (self.blockchain().get_block_timestamp() - stake_node.stake_timestamp) / stake_node.locking_timestamp * (TOTAL_PERCENTAGE - stake_node.tax) / TOTAL_PERCENTAGE;

            return (false, reward_amount);
        }

        (true, reward_amount)
    }

    // rewrad will be calculated in daily basis
    // if 24 hours not passed, days will be zero and reward will also be zero
    #[inline]
    fn calculate_reward(
        &self,
        base_amount: BigUint,
        reward_available_interval: u64,
        apy: u32
    ) -> BigUint {
        base_amount * &BigUint::from(apy) * &BigUint::from(reward_available_interval) / &BigUint::from(TOTAL_PERCENTAGE) / &BigUint::from(YEAR_IN_DAYS * DAY_IN_SECONDS)
    }

    fn require_activation(&self) {
        require!(
            !self.paused().get(),
            "staking is paused"
        );
    }

    /// view
    
    #[view(getApyOfStaker)]
    fn get_apy_of_staker(
        &self,
        caller: &ManagedAddress,
        stake_node_id: u32,
    ) -> u32  {
        let stake_node = self.nodes(caller, stake_node_id).get();
        let mut apy = stake_node.apy + self.referred_count(caller).get() * self.apy_increase_per_referral().get();

        // if referral is activated, it means caller used promo and staked more than referral_activate_amount
        // apy will be increased by promo_increase_apy
        if self.referral_activated(caller).get() {
            apy += self.promo_increase_apy().get();
        }

        apy
    }
    
    #[view(getNodesPerStaker)]
    fn get_nodes_per_staker(
        &self,
        caller: ManagedAddress
    ) -> MultiValueEncoded<StakeNode<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();
        for node_id in self.node_ids(&caller).iter() {
            let mut stake_node = self.nodes(&caller, node_id).get();

            // not unstaked
            if stake_node.state < 3 {
                let (claimable, reward_amount) = self.get_claimable_and_reward(&caller, node_id);
                stake_node.state = match claimable {
                    false => 1, // not-unstakeable
                    true => 2, // unstakeable
                };
                stake_node.reward_amount = reward_amount;
            } else {
                stake_node.state = if self.blockchain().get_block_timestamp() < stake_node.unstake_timestamp + stake_node.delegation_timestamp {
                    3   // unstaked, not-claimable
                } else {
                    4   // claimable
                };
            };
            items_vec.push(stake_node);
        }

        items_vec
    }


    #[view(getActivatedReferrerAddresses)]
    fn get_activated_referrer_addresses(&self) -> MultiValueEncoded<ManagedAddress> {
        let mut items_vec = MultiValueEncoded::new();
        for staker_address in self.staker_addresses().iter() {
            if self.referral_activated(&staker_address).get() {
                items_vec.push(self.referrer_address(&staker_address).get());
            }
        }

        items_vec
    }
}