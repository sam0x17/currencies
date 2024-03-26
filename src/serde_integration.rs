use core::{fmt, marker::PhantomData, str::FromStr};

#[cfg(test)]
use crate::{
    currency::{ETH, USD},
    safety::{Checked, Unchecked},
};

use super::*;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

impl<C: Currency, Safety: safety::Safety> Serialize for Amount<C, Safety> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

struct AmountVisitor<C: Currency, Safety: safety::Safety>(PhantomData<(C, Safety)>);

impl<C: Currency, Safety: safety::Safety> AmountVisitor<C, Safety> {
    fn new() -> Self {
        AmountVisitor(PhantomData)
    }
}

impl<'de, C: Currency, Safety: safety::Safety> Visitor<'de> for AmountVisitor<C, Safety>
where
    C: Currency,
    Safety: safety::Safety,
    Amount<C, Safety>: FromStr<Err = de::value::Error>,
{
    type Value = Amount<C, Safety>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a value that can be converted into an Amount")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse::<Amount<C, Safety>>().map_err(de::Error::custom)
    }
}

impl<'de, C: Currency, Safety: safety::Safety> Deserialize<'de> for Amount<C, Safety>
where
    C: Currency,
    Safety: safety::Safety,
    Amount<C, Safety>: FromStr<Err = de::value::Error>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AmountVisitor::<C, Safety>::new())
    }
}

#[test]
fn serialize_deserialize_unchecked() {
    let amount = Amount::<USD, Unchecked>::from_raw(12345); // $123.45
    let serialized = serde_json::to_string(&amount).expect("Failed to serialize");
    assert_eq!(serialized, "\"$123.45\"");

    // let deserialized: Amount<USD, Unchecked> = serde_json::from_str(&serialized).expect("Failed to deserialize");
    // assert_eq!(deserialized, amount);
}

// Tests serialization and deserialization for an Amount with checked safety.
#[test]
fn serialize_deserialize_checked() {
    let amount = Amount::<USD, Checked>::from_raw(67890); // $678.90
    let serialized = serde_json::to_string(&amount).expect("Failed to serialize");
    assert_eq!(serialized, "\"$678.90\"");

    // let deserialized: Amount<USD, Checked> = serde_json::from_str(&serialized).expect("Failed to deserialize");
    // assert_eq!(deserialized, amount);
}

// Optional: Test serialization and deserialization with other currencies or complex scenarios
// e.g., ETH with a large number of decimal places.
#[test]
fn serialize_deserialize_eth() {
    let amount = Amount::<ETH, Unchecked>::from_raw(123456789012345678901234567890u128.into()); // 123456789.012345678901234567890 ETH
    let serialized = serde_json::to_string(&amount).expect("Failed to serialize");
    assert_eq!(serialized, "\"123456789012.345678901234567890 ETH\"");

    // let deserialized: Amount<ETH, Unchecked> = serde_json::from_str(&serialized).expect("Failed to deserialize");
    // assert_eq!(deserialized, amount);
}
