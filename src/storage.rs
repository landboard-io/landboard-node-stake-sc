elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{StakeType, StakeNode};


const TOTAL_PERCENTAGE: u32 = 10000; // 100%

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
    fn add_stake_types(&self, #[var_args] stake_types: MultiValueEncoded<MultiValue5<u64, u64, BigUint, u32, u32>>) {
        for item in stake_types.into_iter() {
            let (locking_timestamp, delegation_timestamp, min_stake_limit, tax, apy) = item.into_tuple();

            require!(
                tax <= TOTAL_PERCENTAGE,
                "tax cannot be greater than TOTAL_PERCENTAGE 1000"
            );

            let stake_type_id = (self.stake_types().len() + 1) as u32;
            let new_stake_type = StakeType {
                stake_type_id,
                locking_timestamp,
                delegation_timestamp,
                min_stake_limit,
                tax,
                apy,
                disabled: false,
            };
            self.stake_types().push(&new_stake_type);
        }
    }

    #[only_owner]
    #[endpoint(disableStakeTypes)]
    fn disable_stake_types(&self, #[var_args] stake_type_ids: MultiValueEncoded<u32>) {
        for stake_type_id in stake_type_ids.into_iter() {
            let mut stake_type = self.stake_types().get(stake_type_id as usize);
            stake_type.disabled = true;
            self.stake_types().set(stake_type_id as usize, &stake_type);
        }
    }

    #[only_owner]
    #[endpoint(enableStakeTypes)]
    fn enable_stake_types(&self, #[var_args] stake_type_ids: MultiValueEncoded<u32>) {
        for stake_type_id in stake_type_ids.into_iter() {
            let mut stake_type = self.stake_types().get(stake_type_id as usize);
            stake_type.disabled = false;
            self.stake_types().set(stake_type_id as usize, &stake_type);
        }
    }

    #[only_owner]
    #[endpoint(changeStakeType)]
    fn change_stake_type(&self, stake_type_id: u32, locking_timestamp: u64, delegation_timestamp: u64, min_stake_limit: BigUint, tax: u32, apy: u32) {
        require!(
            !self.stake_types().item_is_empty(stake_type_id as usize),
            "stake_type_id does not exist"
        );

        let mut stake_type = self.stake_types().get(stake_type_id as usize);
        stake_type.locking_timestamp = locking_timestamp;
        stake_type.delegation_timestamp = delegation_timestamp;
        stake_type.min_stake_limit = min_stake_limit;
        stake_type.tax = tax;
        stake_type.apy = apy;
        self.stake_types().set(stake_type_id as usize, &stake_type);
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
    fn node_ids(&self, staker_address: &ManagedAddress) -> SetMapper<u32>;

    #[view(getLastNodeId)]
    #[storage_mapper("last_node_id")]
    fn last_node_id(&self, staker_address: &ManagedAddress) -> SingleValueMapper<u32>;

    #[view(getNode)]
    #[storage_mapper("nodes")]
    fn nodes(&self, staker_address: &ManagedAddress, node_id: u32) -> SingleValueMapper<StakeNode<Self::Api>>;

    
    /// referral
    
    #[view(getReferrerAddress)]
    #[storage_mapper("referrer_address")]
    fn referrer_address(&self, user_address: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;

    #[view(getReferralActivated)]
    #[storage_mapper("referral_activated")]
    fn referral_activated(&self, user_address: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getReferredCount)]
    #[storage_mapper("referred_count")]
    fn referred_count(&self, user_address: &ManagedAddress) -> SingleValueMapper<u32>;

    //
    #[view(getReferralActivationAmount)]
    #[storage_mapper("referral_activation_amount")]
    fn referral_activation_amount(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setReferralActivationAmount)]
    fn set_referral_activation_amount(&self, referral_activation_amount: BigUint) {
        self.referral_activation_amount().set(referral_activation_amount);
    }

    //
    #[view(getApyIncreasePerReferral)]
    #[storage_mapper("apy_increase_per_referral")]
    fn apy_increase_per_referral(&self) -> SingleValueMapper<u32>;

    #[only_owner]
    #[endpoint(setApyIncreasePerReferral)]
    fn set_apy_increase_per_referral(&self, apy_increase_per_referral: u32) {
        require!(
            apy_increase_per_referral <= TOTAL_PERCENTAGE,
            "cannot be greater than 10000"
        );
        self.apy_increase_per_referral().set(apy_increase_per_referral);
    }

    //
    #[view(getMaxApyIncreaseByReferral)]
    #[storage_mapper("max_apy_increase_by_referral")]
    fn max_apy_increase_by_referral(&self) -> SingleValueMapper<u32>;

    #[only_owner]
    #[endpoint(setMaxApyIncreaseByReferral)]
    fn set_max_apy_increase_by_referral(&self, max_apy_increase_by_referral: u32) {
        require!(
            max_apy_increase_by_referral <= TOTAL_PERCENTAGE,
            "cannot be greater than 10000"
        );
        self.max_apy_increase_by_referral().set(max_apy_increase_by_referral);
    }

    //
    #[view(getPromoIncreaseApy)]
    #[storage_mapper("promo_increase_apy")]
    fn promo_increase_apy(&self) -> SingleValueMapper<u32>;

    #[only_owner]
    #[endpoint(setPromoIncreaseApy)]
    fn set_promo_increase_apy(&self, promo_increase_apy: u32) {
        self.promo_increase_apy().set(promo_increase_apy);
    }

    //
    #[view(getTotalReferralCountLimit)]
    #[storage_mapper("total_referral_count_limit")]
    fn total_referral_count_limit(&self) -> SingleValueMapper<u32>;

    #[only_owner]
    #[endpoint(setTotalReferralCountLimit)]
    fn set_total_referral_count_limit(&self, total_referral_count_limit: u32) {
        self.total_referral_count_limit().set(total_referral_count_limit);
    }

    //
    #[view(getTotalReferralCount)]
    #[storage_mapper("total_referral_count")]
    fn total_referral_count(&self) -> SingleValueMapper<u32>;
}