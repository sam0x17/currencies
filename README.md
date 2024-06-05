# 💰 currencies

[![Crates.io](https://img.shields.io/crates/v/currencies)](https://crates.io/crates/currencies)
[![docs.rs](https://img.shields.io/docsrs/currencies?label=docs)](https://docs.rs/currencies/latest/currencies/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/currencies/ci.yaml)](https://github.com/sam0x17/currencies/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/currencies)](https://github.com/sam0x17/currencies/blob/main/LICENSE)

This crate provides a generic `Currency` and corresponding `Amount` type that can handle basic
arithmetic operations and formatting of arbitrary currencies and cryptocurrencies. Main
features include:

## Features

- Built-in support for all ISO-4217 currencies with proper precision and formatting
- Support for a variety of cryptocurrencies, also with proper underlying data types and
  formatting. Accurate implementations for `ETH`, `BTC`, `DOT`, and a variety of other
  cryptocurrencies are included.
- The ability to specify whether an `Amount` is forced to only make use of unchecked math, or
  not, at compile-time. Normally this is impossible to control since the `core:ops` operators
  are set up such that the checked operators require their unchecked counterparts to be
  implemented on the host type, however I have gone out of my way to make it possible to
  implement unchecked math _only_, and control it easily with a `Amount<ETH, Checked>`-style
  switch. This is extremely desirable for scenarios where panicking could cause a catastrophic
  issue, and the way it is set up, programmers are forced to consume the `Option` returned by
  the checked ops.
- An easy-to-use macro, `define_currency!` that can define new currencies on-the-fly.
- A painstakingly wrapped version of `primitive_types::U256` that implements many more useful
  `num-traits` and `num-integer` traits than what Parity includes with the `num-traits`
  feature, and are often required when working with amounts of a currency.
- All provided currencies implement most useful `num-traits` and `num-integer` traits.
- Thorough testing of all of the above.

## Examples

```rust
#[test]
fn show_off_currency_math() {
    use currency::*;

    let apple_cost = amt!(USD, "$3.24");
    let orange_cost = Amount::<USD>::from_raw(7_97);
    assert!(apple_cost < orange_cost);
    assert!(apple_cost + orange_cost > orange_cost);
    assert_eq!(format!("{}", apple_cost * orange_cost), "$25.82");
    assert_eq!(format!("{}", apple_cost * 3), "$9.72");

    let mut total = amt!(DOT, "57622449841.0000000004 DOT");
    total -= amt!(DOT, "1000.0 DOT");
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
    let drink_cost = amt_checked!(USD, "$6.29");
    let movie_cost = Amount::<USD, Checked>::from_raw(24_99);
    let Some(outing_cost) = drink_cost + movie_cost else {
      unimplemented!("compiler forces you to handle this!")
    };
    assert_eq!(format!("{}", outing_cost), "$31.28");
}
```

## Future Work
- Additional macros for defining an `Amount` via a decimal literal
- Currency conversion facilities, possibly including an online data source
- Add `Signedness` support to `Amount`
- Additional testing
- Support for negative amounts via an additional const generic defaulting to `Positive`
