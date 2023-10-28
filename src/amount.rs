use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Rem, Sub},
};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, Unsigned, Zero};

use crate::currency::*;

pub use primitive_types::{U256, U512};

/// Automatically implemented on types capable of being used as the "base" / backing type for
/// an [`Amount`] of [`Currency`].
///
/// Must be [`Unsigned`] and implement common [`num_traits`] and basic [`core::ops`] traits
/// (see bounds).
pub trait Base:
    Num
    + Unsigned
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
    + core::fmt::Debug
{
}

impl<
        T: Num
            + Unsigned
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
            + core::fmt::Debug,
    > Base for T
{
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount<C: Currency = USD>(C::Base, PhantomData<C>);

impl<C: Currency> Amount<C> {
    /// Constructs an [`Amount`] from a compatible raw [`Base`] value.
    #[inline]
    pub const fn from_raw(amount: C::Base) -> Self {
        Amount(amount, PhantomData)
    }
}

impl<C: Currency> Rem for Amount<C> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Currency> Div for Amount<C> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Currency> Sub for Amount<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Currency> Add for Amount<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Currency> Mul for Amount<C> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Currency> One for Amount<C> {
    fn one() -> Self {
        todo!()
    }
}

impl<C: Currency> Zero for Amount<C> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<C: Currency> Num for Amount<C> {
    type FromStrRadixErr = <<C as Currency>::Base as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match C::Base::from_str_radix(str, radix) {
            Ok(amount) => Ok(Amount::from_raw(amount)),
            Err(err) => Err(err),
        }
    }
}

#[test]
fn test_from_raw() {
    let a: Amount = Amount::from_raw(1000_00);
    let b: Amount = Amount::from_raw(200_00);
    let c: Amount<USD> = Amount::from_raw(50);
    assert!(a != b);
    assert!(a == a);
    assert!(b != c);
    assert!(a != c);
}
