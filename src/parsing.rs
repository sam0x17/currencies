use core::{fmt::Debug, str::FromStr};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::safety::Unchecked;

use super::*;
use crate::currency::*;

use quoth::{Parsable, ParsableExt, ParseStream, Span, Spanned};

static NUM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+(,\d+)*").unwrap());

/// Represents an [`Amount`] that has been parsed from a string representation. Includes
/// [`Span`] information.
///
/// The parsed [`Amount`] is stored in the `amount` field, and the [`Span`] of the parsed
/// string is stored in the `span` field.
#[derive(Clone, PartialEq, Eq, Hash, Spanned, ParsableExt)]
pub struct ParsedAmount<C: Currency = USD, Safety: safety::Safety = Unchecked> {
    /// The parsed [`Amount`].
    pub amount: Amount<C, Safety>,
    /// The [`Span`] of the parsed [`Amount`].
    pub span: Span,
}

impl<C: Currency, Safety: safety::Safety> Debug for ParsedAmount<C, Safety> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.amount)
    }
}

impl<C: Currency, Safety: safety::Safety> Parsable for ParsedAmount<C, Safety> {
    fn parse(stream: &mut quoth::ParseStream) -> quoth::Result<Self> {
        let start_position = stream.position;
        if C::STYLE == FormatStyle::PrefixAttached || C::STYLE == FormatStyle::PrefixSpaced {
            let _symbol = stream.parse_str(C::SYMBOL)?;
        }
        if C::STYLE == FormatStyle::PrefixSpaced {
            let _space = stream.parse_str(" ")?;
        }
        let whole_start_position = stream.position;
        let whole_part = stream.parse_regex(&*NUM_REGEX)?;
        let _dot = stream.parse_str(".")?;
        let mut decimal_digits = Vec::new();
        let decimal_start_position = stream.position;
        while stream.next_digit().is_ok() {
            decimal_digits.push(stream.parse_digit()?);
        }
        if decimal_digits.len() > C::decimal_digits() {
            return Err(quoth::Error::new(
                Span::new(stream.source().clone(), decimal_start_position..stream.position),
                "too many decimal digits",
            ));
        }
        while decimal_digits.len() < C::decimal_digits() {
            decimal_digits.push(0);
        }
        let dec_end_position = stream.position;
        let raw = format!("{}{}", whole_part, decimal_digits.into_iter().map(|d| d.to_string()).collect::<String>()).replace(",", "");
        println!("raw: {}", raw);
        let backing = C::Backing::from_str(&raw).map_err(|_| {
            quoth::Error::new(
                Span::new(stream.source().clone(), whole_start_position..dec_end_position),
                "invalid amount",
            )
        })?;
        let amount = Amount::from_raw(backing);
        if C::STYLE == FormatStyle::SuffixSpaced {
            let _space = stream.parse_str(" ")?;
        }
        if C::STYLE == FormatStyle::SuffixAttached || C::STYLE == FormatStyle::SuffixSpaced {
            let _symbol = stream.parse_str(C::SYMBOL)?;
        }
        let end_position = stream.position;
        Ok(ParsedAmount {
            amount,
            span: Span::new(stream.source().clone(), start_position..end_position),
        })
    }
}

impl<C: Currency, Safety: safety::Safety> FromStr for Amount<C, Safety> {
    type Err = quoth::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = ParseStream::from(s);
        let parsed = stream.parse::<ParsedAmount<C, Safety>>()?;
        Ok(parsed.amount)
    }
}

#[test]
fn test_parsing_usd() {
    assert_eq!(USD::decimal_digits(), 2);

    let amount: Amount<USD> = "$1,000.00".parse().unwrap();
    assert_eq!(amount, Amount::from_raw(1_000_00));

    assert!("$1,00000".parse::<Amount<USD>>().unwrap_err().to_string().contains("expected `.`"));

    let amount: Amount<USD> = "$2748972.98".parse().unwrap();
    assert_eq!(amount, Amount::from_raw(2748972_98));

    let amount: Amount<USD> = "$0.01".parse().unwrap();
    assert_eq!(amount, Amount::from_raw(0_01));

    assert!("$0.001"
        .parse::<Amount<USD>>()
        .unwrap_err()
        .to_string()
        .contains("too many decimal digits"));

    let amount: Amount<USD> = "$0.1".parse().unwrap();
    assert_eq!(Amount::<USD>::from_raw(010).to_string(), "$0.10");
    assert_eq!(amount.to_string(), "$0.10");
    assert_eq!(amount, Amount::from_raw(010));
}
