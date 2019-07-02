//! # Ethereum
//!
//! A library for generating Ethereum Wallets.

extern crate hex;
extern crate rand;
extern crate secp256k1;
extern crate serde;
extern crate serde_json;
extern crate tiny_keccak;

pub mod address;

#[cfg(feature = "serde")]
pub mod builder;

pub mod keypair;
pub use self::keypair::*;

pub mod utils;
pub use self::utils::*;

#[cfg(feature = "serde")]
pub mod wallet;
#[cfg(feature = "serde")]
pub use self::wallet::*;
