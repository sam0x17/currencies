//! Core functionality for [`currencies`](https://crates.io/crates/currencies) crate, not
//! including proc macros.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

pub mod amount;
pub use amount::{Amount, Backing};
pub mod currency;
pub use currency::Currency;
pub mod u256;
pub use u256::U256;
pub mod safety;

#[cfg(feature = "parsing")]
mod parsing;
#[cfg(feature = "parsing")]
pub use parsing::*;

/// Contains impls for [`serde`] integration
#[cfg(feature = "serde")]
pub mod serde_integration;

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

    let mut total = Amount::<DOT>::from_raw(5762244984_10000000004u128.into());
    total -= Amount::from_raw(1000_0000000000u128.into());
    total *= Amount::from_raw(2_0000000000u64.into());
    assert_eq!(format!("{}", total), "115244897682.0000000008 DOT");
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
