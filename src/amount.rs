use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Unsigned, Zero};

use crate::currency::*;

pub use primitive_types::U256;

pub trait Base:
    Unsigned
    + Zero
    + One
    + CheckedAdd
    + CheckedSub
    + CheckedDiv
    + CheckedMul
    + Mul
    + Add
    + Sub
    + Div
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
    + Clone
    + core::hash::Hash
{
}

impl<
        T: Unsigned
            + Zero
            + One
            + CheckedAdd
            + CheckedSub
            + CheckedDiv
            + CheckedMul
            + Mul
            + Add
            + Sub
            + Div
            + PartialEq
            + Eq
            + PartialOrd
            + Ord
            + Copy
            + Clone
            + core::hash::Hash,
    > Base for T
{
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount<C: Currency = USD>(C::Base, PhantomData<C>);

impl<C: Currency> Amount<C> {
    /// Constructs an [`Amount`] from a compatible raw [`Base`] value.
    pub const fn from_raw(amount: C::Base) -> Self {
        Amount(amount, PhantomData)
    }
}

#[test]
fn test_types() {
    let a: Amount = Amount::from_raw(1000_00);
    let b: Amount = Amount::from_raw(200_00);
    let c: Amount<USD> = Amount::from_raw(50);
    assert!(a != b);
    assert!(a == a);
    assert!(b != c);
    assert!(a != c);
}
