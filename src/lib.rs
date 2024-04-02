#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

pub mod cep47;
mod cep47_tests;
pub mod data;
pub mod events;
#[cfg(target_arch = "wasm32")]
mod legacy_events;
