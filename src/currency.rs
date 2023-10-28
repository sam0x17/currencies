use crate::amount::*;

pub trait Currency: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {
    /// Represents the underlying (signed or un-signed) primitive integer type used to
    /// represent [`Amount`]s of this [`Currency`].
    type Base: Base;

    /// Determines the number of supported fractional decimal digits for [`Amount`]s of this
    /// [`Currency`] (a value of 0 means only integers can be represented). This is also the
    /// position of the decimal point from the RHS of the underlying [`Base`].
    const FRAC_DIGITS: usize;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct USD;

impl Currency for USD {
    type Base = u64;
    const FRAC_DIGITS: usize = 2;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ETH;

impl Currency for ETH {
    type Base = U256;
    const FRAC_DIGITS: usize = 18;
}
