#![no_std]

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

    let mut total = Amount::<AAVE>::from_raw(5762244984_100000000000000004u128.into());
    total -= Amount::from_raw(1000_000000000000000000u128.into());
    total *= Amount::from_raw(2_000000000000000000u64.into());
    assert_eq!(format!("{}", total), "11524487968.200000000000000008 AAVE");
}
