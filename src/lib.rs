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

#![no_std]
#![warn(missing_docs)]

pub mod amount;
pub use amount::{Amount, Backing};
pub mod currency;
pub use currency::Currency;
mod u256;
pub use u256::U256;
pub mod safety;

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
use alloc::format;

#[test]
fn show_off_currency_math() {
    use currency::*;

    let apple_cost = Amount::<USD>::from_raw(3_24);
    let orange_cost = Amount::<USD>::from_raw(7_97);
    assert!(apple_cost < orange_cost);
    assert!(apple_cost + orange_cost > orange_cost);
    assert_eq!(format!("{}", apple_cost * orange_cost), "$25.82");
    assert_eq!(format!("{}", apple_cost * 3), "$9.72");

    let mut total = Amount::<AAVE>::from_raw(5762244984_100000000000000004u128.into());
    total -= Amount::from_raw(1000_000000000000000000u128.into());
    total *= Amount::from_raw(2_000000000000000000u64.into());
    assert_eq!(format!("{}", total), "11524487968.200000000000000008 AAVE");
}

#[test]
fn show_off_checked_math() {
    use currency::*;
    use safety::*;

    // When using currency amounts with `Safety = Checked`, the Amount struct has been specially set
    // up so that only checked math will be allowed, and you can still use the normal
    // operator-based syntax. Thus currency amounts like this should never panic and are
    // suitable for use in critical/infallible environments.

    let drink_cost = Amount::<USD, Checked>::from_raw(6_29);
    let movie_cost = Amount::<USD, Checked>::from_raw(24_99);
    let Some(outing_cost) = drink_cost + movie_cost else {
        unimplemented!("compiler forces you to handle this!")
    };
    assert_eq!(format!("{}", outing_cost), "$31.28");
}
