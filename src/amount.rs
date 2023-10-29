//! Home of the [`Amount`] struct and supporting types and impls.

use core::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Shl, Shr, Sub, SubAssign},
};
use num_integer::Integer;
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, PrimInt, ToPrimitive,
    Unsigned, Zero,
};

use crate::currency::*;
use crate::safety::{self, *};

/// Automatically implemented on types capable of being used as the "base" / backing type for
/// an [`Amount`] of [`Currency`].
///
/// Must be [`Unsigned`] and implement common [`num_traits`] and basic [`core::ops`] traits
/// (see bounds).
pub trait Backing:
    Num
    + Unsigned
    + Zero
    + One
    + Integer
    + CheckedAdd
    + CheckedSub
    + CheckedDiv
    + CheckedMul
    + Mul
    + Add
    + Sub
    + Div
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
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
    + core::fmt::Display
    + TrailingZeros
    + From<u32>
    + ToPrimitive
{
}

impl<
        T: Num
            + Unsigned
            + Zero
            + One
            + Integer
            + CheckedAdd
            + CheckedSub
            + CheckedDiv
            + CheckedMul
            + Mul
            + Add
            + Sub
            + Div
            + AddAssign
            + SubAssign
            + MulAssign
            + DivAssign
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
            + core::fmt::Display
            + TrailingZeros
            + From<u32>
            + ToPrimitive,
    > Backing for T
{
}

/// Provides access to [`TrailingZeros::trailing_zeros`] without needing to impl [`PrimInt`].
pub trait TrailingZeros {
    /// Returns the number of trailing zeros at the end of `self`.
    fn trailing_zeros(&self) -> u32;
}

impl<T: PrimInt> TrailingZeros for T {
    fn trailing_zeros(&self) -> u32 {
        PrimInt::trailing_zeros(*self)
    }
}

/// Generically represents an amount of a specified [`Currency`].
///
/// Setting `Self::Safety` to [`Unchecked`] will allow for full use of all supported math
/// operators, but allows for things like overflowing, division by zero, that can lead to
/// panics during runtime.
///
/// Setting `Self::Safety` to [`Checked`] replaces basic arithmetic operators with their
/// checked counterparts that can never panic but typically return an [`Option`] or [`Result`]
/// that must be used. This should make usages of this [`Amount`] 100% safe to use in
/// situations where panicking is dangerous.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount<C: Currency = USD, Safety: safety::Safety = Unchecked>(
    C::Backing,
    PhantomData<C>,
    PhantomData<Safety>,
    // TODO: eventually we could add Signedness here
);

impl<C: Currency, Safety: safety::Safety> core::fmt::Display for Amount<C, Safety> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let decimals = C::BASE.trailing_zeros() as usize;

        let major = self.0 / C::BASE;
        let minor = self.0 % C::BASE;

        // handle formatting for prefix-style currencies
        match C::STYLE {
            FormatStyle::PrefixAttached => write!(f, "{}", C::SYMBOL)?,
            FormatStyle::PrefixSpaced => write!(f, "{} ", C::SYMBOL)?,
            _ => (),
        }

        // avoids allocation
        write!(f, "{}.", major)?;

        // Collect the minor digits into an array, and then print them in reverse order
        let mut minor_digits = [0u8; 64]; // HACK: max size needed for a U512, increase this to support larger types
        let mut minor_val = minor;
        for i in 0..decimals {
            let digit = minor_val % C::Backing::from(10);
            minor_digits[decimals - 1 - i] = digit.to_u64().unwrap() as u8;
            minor_val /= C::Backing::from(10);
        }
        for &digit in &minor_digits[..decimals] {
            // avoids allocation
            core::fmt::Write::write_char(f, char::from_digit(digit as u32, 10).unwrap())?;
        }

        // handle formatting for suffix-style currencies
        match C::STYLE {
            FormatStyle::SuffixAttached => write!(f, "{}", C::SYMBOL)?,
            FormatStyle::SuffixSpaced => write!(f, " {}", C::SYMBOL)?,
            _ => (),
        }

        Ok(())
    }
}

impl<C: Currency, Safety: safety::Safety> core::fmt::Debug for Amount<C, Safety> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<C: Currency, Safety: safety::Safety> Amount<C, Safety> {
    /// Constructs an [`Amount`] from a compatible raw [`Backing`] value.
    #[inline]
    pub const fn from_raw(amount: C::Backing) -> Self {
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
    type Output = C::Backing;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0)
    }
}

impl<C: Currency> Div for Amount<C, Checked> {
    type Output = Option<C::Backing>;

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

impl<C: Currency> SubAssign for Amount<C, Unchecked> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

impl<C: Currency> AddAssign for Amount<C, Unchecked> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C: Currency> Mul for Amount<C, Unchecked> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0 * rhs.0 / C::BASE)
    }
}

impl<C: Currency> Mul for Amount<C, Checked> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        Some(Self::from_raw(
            self.0.checked_mul(&rhs.0)?.checked_div(&C::BASE)?,
        ))
    }
}

impl<C: Currency> Mul<u8> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_u8(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<u16> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_u16(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<u32> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_u32(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<u64> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_u64(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<u128> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: u128) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_u128(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<usize> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_usize(rhs).unwrap()))
    }
}

impl<C: Currency> Mul<i32> for Amount<C, Unchecked>
where
    C::Backing: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::from_raw(self.0.mul(C::Backing::from_i32(rhs).unwrap()))
    }
}

impl<C: Currency> MulAssign for Amount<C, Unchecked> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<C: Currency> One for Amount<C, Unchecked> {
    fn one() -> Self {
        Self::from_raw(<C as Currency>::Backing::one())
    }
}

impl<C: Currency> Zero for Amount<C, Unchecked> {
    fn zero() -> Self {
        Self::from_raw(<C as Currency>::Backing::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

#[cfg(test)]
use crate::*;

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
fn test_basic_ops_unchecked() {
    let a = Amount::<USD>::from_raw(100_00);
    let b = Amount::<USD>::from_raw(50_00);
    assert!(a + b == Amount::from_raw(150_00));
    assert!(a / b == 2);
    assert!(a * b == Amount::from_raw(5000_00));
    let mut c = a;
    c += b;
    assert!(c == Amount::from_raw(150_00));
    assert!(a * 3u8 == Amount::from_raw(300_00));
    assert!(a * 3u16 == Amount::from_raw(300_00));
    assert!(a * 3u32 == Amount::from_raw(300_00));
    assert!(a * 3u64 == Amount::from_raw(300_00));
    assert!(a * 3u128 == Amount::from_raw(300_00));
    assert!(a * 3usize == Amount::from_raw(300_00));
    let a = Amount::<USD>::from_raw(u64::MAX);
    assert!(a - Amount::<USD>::from_raw(0_01) < a);
    assert!(a - Amount::<USD>::from_raw(0_01) > b);
}

#[test]
fn test_basic_ops_checked() {
    let a = Amount::<USD, Checked>::from_raw(33_26);
    let b = Amount::<USD, Checked>::from_raw(245_23);
    assert!((a - b).is_none());
    assert!((a + b).unwrap() == Amount::from_raw(278_49));
    assert!((a / b).unwrap() == 0);
    assert!((b / a).unwrap() == 7);
    assert!((a / Amount::from_raw(0)).is_none());
    let a = Amount::<ETH, Checked>::from_raw(U256::MAX_VALUE);
    assert!((a + Amount::from_raw(U256::from(1))).is_none());
    assert!((a - Amount::from_raw(U256::from(1))).is_some());
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
use alloc::format;

#[test]
fn test_display() {
    assert_eq!(ETH::BASE.trailing_zeros(), 18);
    let a = Amount::<USD>::from_raw(124_27);
    let b = Amount::<ETH>::from_raw(500000000_000000000000000001u128.into());
    let c = Amount::<AUD>::from_raw(365000000_23);
    let d = Amount::<DOT>::from_raw(457_0000000003);
    let e = Amount::<KSM>::from_raw(249879873_700000000004);
    assert_eq!(format!("{}", a), "$124.27");
    assert_eq!(format!("{}", b), "500000000.000000000000000001 ETH");
    assert_eq!(format!("{}", c), "$365000000.23");
    assert_eq!(format!("{}", d), "457.0000000003 DOT");
    assert_eq!(format!("{}", e), "249879873.700000000004 KSM");
}
