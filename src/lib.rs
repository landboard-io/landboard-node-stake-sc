#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::derive::contract]
pub trait LandboardStaking{
    #[init]
    fn init(&self) {
        
    }
}