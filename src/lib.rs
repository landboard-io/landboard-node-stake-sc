#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod state;
mod storage;

// use crate::state::StakeType;
// use crate::state::StakeNode;

#[elrond_wasm::derive::contract]
pub trait LandboardStaking: storage::StorageModule{
    #[init]
    fn init(&self) {
        
    }
}