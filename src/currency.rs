use crate::amount::*;

pub trait Currency: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {
    /// Represents the underlying (signed or un-signed) primitive integer type used to
    /// represent [`Amount`]s of this [`Currency`].
    type Base: Base;

    /// Determines the number of supported fractional decimal digits for [`Amount`]s of this
    /// [`Currency`] (a value of 0 means only integers can be represented). This is also the
    /// position of the decimal point from the RHS of the underlying [`Base`].
    const FRAC_DIGITS: usize;

    /// When set to true, only checked math operations are allowed on amounts of this
    /// [`Currency`], making it impossible to have unhandled underflow/overflow errors.
    const SAFE: bool;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct USD<const SAFE: bool = false, const SIGNED: bool = true>;

impl<const SAFE: bool, const SIGNED: bool> Currency for USD<SAFE, SIGNED> {
    type Base = u64;
    const FRAC_DIGITS: usize = 2;
    const SAFE: bool = SAFE;
}
