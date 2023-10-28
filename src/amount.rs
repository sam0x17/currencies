use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Rem, Shl, Shr, Sub},
};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, Unsigned, Zero};

use crate::currency::*;
use crate::safety::{self, *};

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
    + Shl
    + Shr
    + Shl<usize>
    + Shr<usize>
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
            + Shl
            + Shr
            + Shl<usize>
            + Shr<usize>
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
pub struct Amount<C: Currency = USD, Safety: safety::Safety = Unchecked>(
    C::Base,
    PhantomData<C>,
    PhantomData<Safety>,
);

impl<C: Currency, Safety: safety::Safety> Amount<C, Safety> {
    /// Constructs an [`Amount`] from a compatible raw [`Base`] value.
    #[inline]
    pub const fn from_raw(amount: C::Base) -> Self {
        Amount(amount, PhantomData, PhantomData)
    }
}

impl<C: Currency, Safety: safety::Safety> Rem for Amount<C, Safety> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0.rem(rhs.0))
    }
}

impl<C: Currency> Div for Amount<C, Unchecked> {
    type Output = C::Base;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0)
    }
}

impl<C: Currency> Div for Amount<C, Checked> {
    type Output = Option<C::Base>;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.checked_div(&rhs.0)
    }
}

impl<C: Currency> Sub for Amount<C, Unchecked> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0.sub(rhs.0))
    }
}

impl<C: Currency> Sub for Amount<C, Checked> {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.0.checked_sub(&rhs.0) {
            Some(res) => Some(Self::from_raw(res)),
            None => None,
        }
    }
}

impl<C: Currency> Add for Amount<C, Unchecked> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0.add(rhs.0))
    }
}

impl<C: Currency> Add for Amount<C, Checked> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match self.0.checked_add(&rhs.0) {
            Some(res) => Some(Self::from_raw(res)),
            None => None,
        }
    }
}

impl<C: Currency> Mul for Amount<C, Unchecked> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0.mul(rhs.0))
    }
}

impl<C: Currency> Mul for Amount<C, Checked> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self.0.checked_mul(&rhs.0) {
            Some(res) => Some(Self::from_raw(res)),
            None => None,
        }
    }
}

impl<C: Currency> One for Amount<C, Unchecked> {
    fn one() -> Self {
        Self::from_raw(<C as Currency>::Base::one())
    }
}

impl<C: Currency> Zero for Amount<C, Unchecked> {
    fn zero() -> Self {
        Self::from_raw(<C as Currency>::Base::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
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

#[test]
fn test_basic_ops() {
    let a = Amount::<USD>::from_raw(100_00);
    let b = Amount::<USD>::from_raw(50_00);
    assert!(a + b == Amount::<USD>::from_raw(150_00));
    assert!(a / b == 2);
    assert!(a * b == Amount::<USD>::from_raw(500000_00));
    let a = Amount::<USD>::from_raw(u64::MAX);
    assert!(a - Amount::<USD>::from_raw(0_01) < a);
    assert!(a - Amount::<USD>::from_raw(0_01) > b);
}

#[test]
fn test_basic_checked_ops() {
    let a = Amount::<USD, Checked>::from_raw(33_26);
    let b = Amount::<USD, Checked>::from_raw(245_23);
    assert!((a - b).is_none());
    assert!((a + b).unwrap() == Amount::from_raw(278_49));
    assert!((a / b).unwrap() == 0);
    assert!((b / a).unwrap() == 7);
    assert!((a / Amount::from_raw(0)).is_none());
    let a = Amount::<ETH, Checked>::from_raw(U256::max_value());
    assert!((a + Amount::from_raw(U256::from(1))).is_none());
    assert!((a - Amount::from_raw(U256::from(1))).is_some());
}
