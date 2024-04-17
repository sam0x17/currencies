//! # 💰 currencies
//!
//! [![Crates.io](https://img.shields.io/crates/v/currencies)](https://crates.io/crates/currencies)
//! [![docs.rs](https://img.shields.io/docsrs/currencies?label=docs)](https://docs.rs/currencies/latest/currencies/)
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/currencies/ci.yaml)](https://github.com/sam0x17/currencies/actions/workflows/ci.yaml?query=branch%3Amain)
//! [![MIT License](https://img.shields.io/github/license/sam0x17/currencies)](https://github.com/sam0x17/currencies/blob/main/LICENSE)
//!
//! This crate allows for generic manipulation of currencies (both real-world and
//! cryptocurrencies) via the [`Amount`] struct and the [`Currency`] trait.
//!
//! The [`Amount`] struct is able to represent arbitrary amounts of any supported [`Currency`]
//! with the ability to restrict the underlying currencies at compile-time to only allow
//! checked arithmetic operations and requires consuming an [`Option`] in all fallible
//! circumstances.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

pub use currencies_core::*;
pub use currencies_macros::*;
