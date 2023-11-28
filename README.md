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
  formatting. Accurate implementations for `ETH`, `BTC`, `DOT`, `KSM` and `AAVE` are included.
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

## Future Work
- Additional macros for defining an `Amount` via a decimal literal
- Currency conversion facilities, possibly including an online data source
- Add `Signedness` support to `Amount`
- Additional testing
- Support for negative amounts via an additional const generic defaulting to `Positive`
