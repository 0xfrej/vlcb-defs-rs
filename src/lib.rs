#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![deny(unsafe_code)]

#[cfg(test)]
extern crate alloc;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs"));

pub mod module;
pub mod platform;
pub mod dcc;
pub mod fast_clock;
pub mod vlcb;
pub mod can;
pub mod macros;