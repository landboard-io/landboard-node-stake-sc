elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{StakeType, StakeNode};

#[elrond_wasm::module]
pub trait StorageModule {
    #[view(getStakeTypes)]
    #[storage_mapper("stake_types")]
    fn stake_types(&self) -> VecMapper<StakeType<Self::Api>>;

    #[view(getStakerAddresses)]
    #[storage_mapper("staker_addresses")]
    fn staker_addresses(&self) -> VecMapper<ManagedAddress>;

    #[view(getNodeIds)]
    #[storage_mapper("node_ids")]
    fn node_ids(&self, staker_address: &ManagedAddress) -> SetMapper<u32>;

    #[view(getNode)]
    #[storage_mapper("nodes")]
    fn nodes(&self, staker_address: &ManagedAddress, node_id: u32) -> SingleValueMapper<StakeNode<Self::Api>>;
}