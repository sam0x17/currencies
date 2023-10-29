use crate::amount::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FormatStyle {
    /// Specifies that the symbol should prefix the amount with no space, like "$40.00". Common
    /// for major English-speaking locales.
    PrefixAttached,
    /// Specifies that the symbol should suffix the amount with no space, like  "40.00€".
    /// Commonly used in non-English locales.
    SuffixAttached,
    /// Specifies that the symbol should prefix the amount with a space, like "$ 40.00". Generally not used.
    PrefixSpaced,
    /// Specifies that the symbol should suffix the amount with a space, like "40.00 AUD".
    /// Commonly used by currencies that use a multi-character or alphabetic symbol.
    SuffixSpaced,
}

impl Default for FormatStyle {
    fn default() -> Self {
        Self::PrefixAttached
    }
}

/// Uniquely defines a particular currency, such as [`USD`], [`BTC`], or [`ETH`].
pub trait Currency: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {
    /// Represents the underlying (signed or un-signed) primitive integer type used to
    /// represent [`Amount`]s of this [`Currency`].
    type Base: Base;

    /// Determines the number of supported fractional decimal digits that will be supported by
    /// an [`Amount`] of this [`Currency`] (a value of 0 means only integers can be
    /// represented). This is also the position of the decimal point from the RHS of the
    /// underlying [`Base`].
    const FRAC_DIGITS: usize;

    /// Specifies a 3-4 digit acronym or "code" that can be used as a short name for this
    /// [`Currency`]. For ISO-supported currencies, this will be equal to the ISO-4217
    /// alphabetic code, which can be found here: <https://en.wikipedia.org/wiki/ISO_4217>.
    ///
    /// For cryptocurrencies and other currencies not named in ISO-4217, this should be a short
    /// globally unique code that is specific to the currency, for example `BTC` for Bitcoin,
    /// `ETH` for Ethereum, etc..
    const CODE: &'static str;

    /// Specifies the monetary _symbol_, such as `$`, that is commonly associated with this
    /// [`Currency`].
    ///
    /// It is worth noting that such symbols can be multiple characters long, are not globally
    /// unique (actually many currencies use the `$` symbol and there are plenty examples of
    /// the same symbol being used for many currencies), and are not governed or defined by
    /// ISO-4217 or any other ISO. They also do not have to be symbols, they could be a word or
    /// several words long.
    ///
    /// The symbol is used when formatted values of an [`Amount`] using this [`Currency`] are
    /// displayed.
    const SYMBOL: &'static str;

    /// Specifies a long-hand / "proper" name for this [`Currency`], for example "United States
    /// Dollar".
    ///
    /// For currencies governed by ISO-4217, this corresponds with the "entity" field.
    const PROPER_NAME: &'static str;

    /// Specifies how an [`Amount`] of this [`Currency`] should be displayed when it is
    /// represented textually via [`core::fmt::Display`] and [`core::fmt::Debug`].
    const STYLE: FormatStyle;

    /// Set to `true` if this [`Currency`] is governed by ISO-4217. Otherwise `false`.
    const IS_ISO: bool;

    /// Set to `true` if this [`Currency`] is a cryptocurrency. Otherwise `false`.
    ///
    /// This is provided separately from `IS_ISO` to prepare for a future where one or more
    /// cryptocurrencies are included in ISO-4217.
    const IS_CRYPTO: bool;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct USD;

impl Currency for USD {
    type Base = u64;
    const FRAC_DIGITS: usize = 2;
    const CODE: &'static str = "USD";
    const SYMBOL: &'static str = "$";
    const PROPER_NAME: &'static str = "United States Dollar";
    const STYLE: FormatStyle = FormatStyle::PrefixAttached;
    const IS_ISO: bool = true;
    const IS_CRYPTO: bool = false;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ETH;

impl Currency for ETH {
    type Base = U256;
    const FRAC_DIGITS: usize = 18;
    const CODE: &'static str = "ETH";
    const SYMBOL: &'static str = "ETH"; // "Ξ" is occasionally used, but "ETH" is much more common
    const PROPER_NAME: &'static str = "Ethereum";
    const STYLE: FormatStyle = FormatStyle::PrefixAttached;
    const IS_ISO: bool = false;
    const IS_CRYPTO: bool = true;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTC;

impl Currency for BTC {
    type Base = U256;
    const FRAC_DIGITS: usize = 8;
    const CODE: &'static str = "BTC";
    const SYMBOL: &'static str = "BTC"; // The official Unicode character for Bitcoin is "₿", but often "BTC" is more recognized.
    const PROPER_NAME: &'static str = "Bitcoin";
    const STYLE: FormatStyle = FormatStyle::SuffixSpaced; // Commonly formatted as "0.001 BTC"
    const IS_ISO: bool = false;
    const IS_CRYPTO: bool = true;
}
