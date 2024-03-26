use core::{fmt, marker::PhantomData, str::FromStr};

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
