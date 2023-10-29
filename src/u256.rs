use core::ops::*;
use num_integer::Integer;
use num_traits::*;

/// Wraps [`primitive_types::U256`] enhancing it with some extra trait impls needed for
/// currency manipulation.
///
/// I have submitted an issue upstream to see if we can get some additional [`num_integer`] and
/// [`num_traits`] trait impls added.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub primitive_types::U256);

impl U256 {
    pub const MAX_VALUE: U256 = U256(primitive_types::U256::MAX);
}

impl core::fmt::Display for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Const function capable of constructing a [`U256`] from a [`u64`], useful for specifying
/// [`Currency::BASE`] for currencies have a [`Currency::Backing`] set to [`U256`].
pub const fn u64_to_u256(n: u64) -> U256 {
    U256(primitive_types::U256([n, 0, 0, 0]))
}

impl Zero for U256 {
    fn zero() -> Self {
        U256(primitive_types::U256([0, 0, 0, 0]))
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for U256 {
    fn one() -> Self {
        U256(primitive_types::U256([1, 0, 0, 0]))
    }
}

impl Unsigned for U256 {}

impl Shr<usize> for U256 {
    type Output = U256;

    fn shr(self, rhs: usize) -> Self::Output {
        U256(self.0.shr(rhs))
    }
}

impl Shl<usize> for U256 {
    type Output = U256;

    fn shl(self, rhs: usize) -> Self::Output {
        U256(self.0.shl(rhs))
    }
}

impl Shr for U256 {
    type Output = U256;

    fn shr(self, rhs: Self) -> Self::Output {
        U256(self.0.shr(rhs.0))
    }
}

impl Shl for U256 {
    type Output = U256;

    fn shl(self, rhs: Self) -> Self::Output {
        U256(self.0.shl(rhs.0))
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: Self) -> Self::Output {
        U256(self.0.add(rhs.0))
    }
}

impl Sub for U256 {
    type Output = U256;

    fn sub(self, rhs: Self) -> Self::Output {
        U256(self.0.sub(rhs.0))
    }
}

impl Div for U256 {
    type Output = U256;

    fn div(self, rhs: Self) -> Self::Output {
        U256(self.0.div(rhs.0))
    }
}

impl Mul for U256 {
    type Output = U256;

    fn mul(self, rhs: Self) -> Self::Output {
        U256(self.0.mul(rhs.0))
    }
}

impl Rem for U256 {
    type Output = U256;

    fn rem(self, rhs: Self) -> Self::Output {
        U256(self.0.rem(rhs.0))
    }
}

impl CheckedAdd for U256 {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(val) => Some(U256(val)),
            None => None,
        }
    }
}

impl CheckedSub for U256 {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Some(U256(val)),
            None => None,
        }
    }
}

impl CheckedDiv for U256 {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        match self.0.checked_div(rhs.0) {
            Some(val) => Some(U256(val)),
            None => None,
        }
    }
}

impl CheckedMul for U256 {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(val) => Some(U256(val)),
            None => None,
        }
    }
}

impl Num for U256 {
    type FromStrRadixErr = <primitive_types::U256 as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match primitive_types::U256::from_str_radix(str, radix) {
            Ok(val) => Ok(U256(val)),
            Err(err) => Err(err),
        }
    }
}

impl From<u8> for U256 {
    fn from(value: u8) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<u16> for U256 {
    fn from(value: u16) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<u32> for U256 {
    fn from(value: u32) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<i8> for U256 {
    fn from(value: i8) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<i16> for U256 {
    fn from(value: i16) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<i32> for U256 {
    fn from(value: i32) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<i64> for U256 {
    fn from(value: i64) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<i128> for U256 {
    fn from(value: i128) -> Self {
        U256(primitive_types::U256::from(value))
    }
}

impl From<primitive_types::U256> for U256 {
    fn from(value: primitive_types::U256) -> Self {
        U256(value)
    }
}

impl Integer for U256 {
    fn div_floor(&self, other: &Self) -> Self {
        self.0.div(other.0).into()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        self.0.rem(other.0).into()
    }

    fn gcd(&self, other: &Self) -> Self {
        let mut a = *self;
        let mut b = *other;
        while !b.is_zero() {
            let temp = b;
            b = a.mod_floor(&b);
            a = temp;
        }
        a
    }

    fn lcm(&self, other: &Self) -> Self {
        let product = self.0.mul(other.0);
        let gcd_val = self.gcd(other);
        U256(product.div(gcd_val.0))
    }

    fn divides(&self, other: &Self) -> bool {
        !self.is_zero() && other.0.rem(self.0).is_zero()
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        !(*other).is_zero() && (self.rem(*other)).is_zero()
    }

    fn is_even(&self) -> bool {
        self.0.low_u64() & 1 == 0
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self.div_floor(other), self.mod_floor(other))
    }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
use alloc::format;

#[test]
fn test_mod_and_is_zero() {
    let a: U256 = 20.into();
    let b: U256 = 4.into();

    assert_eq!(a % b, U256::zero());
    assert!((a % b).is_zero());
}

#[test]
fn test_div_floor() {
    let a: U256 = 13.into();
    let b: U256 = 5.into();
    let result = a.div_floor(&b);
    assert_eq!(result, 2.into());
}

#[test]
fn test_mod_floor() {
    let a: U256 = 13.into();
    let b: U256 = 5.into();
    let result = a.mod_floor(&b);
    assert_eq!(result, 3.into());
}

#[test]
fn test_gcd() {
    let a: U256 = 56.into();
    let b: U256 = 98.into();
    let result = a.gcd(&b);
    assert_eq!(result, 14.into());
}

#[test]
fn test_lcm() {
    let a: U256 = 4.into();
    let b: U256 = 5.into();
    let result = a.lcm(&b);
    assert_eq!(result, 20.into());
}

#[test]
fn test_divides() {
    let a: U256 = 4.into();
    let b: U256 = 20.into();
    assert!(a.divides(&b));

    let c: U256 = 3.into();
    assert!(!c.divides(&b));

    let a = U256::from(24);
    let b = U256::from(4);
    let c = U256::from(5);
    let d = U256::from(0);
    let e = U256::from(1);

    assert!(b.divides(&a)); // 4 divides 24
    assert!(!c.divides(&a)); // 5 doesn't divide 24
    assert!(!d.divides(&a)); // 0 doesn't divide any number
    assert!(e.divides(&a)); // 1 divides every number
    assert!(a.divides(&d)); // Every number divides 0
}

#[test]
fn test_is_multiple_of() {
    let a: U256 = 20.into();
    let b: U256 = 4.into();
    assert!(a.is_multiple_of(&b));

    let c: U256 = 3.into();
    assert!(!a.is_multiple_of(&c));
}

#[test]
fn test_is_even_odd() {
    let a: U256 = 4.into();
    assert!(a.is_even());
    assert!(!a.is_odd());

    let b: U256 = 3.into();
    assert!(!b.is_even());
    assert!(b.is_odd());
}

#[test]
fn test_div_rem() {
    let a: U256 = 13.into();
    let b: U256 = 5.into();
    let (div, rem) = a.div_rem(&b);
    assert_eq!(div, 2.into());
    assert_eq!(rem, 3.into());
}

#[test]
fn test_modulo_operation() {
    let a: U256 = 20.into();
    let b: U256 = 4.into();

    assert_eq!(a % b, U256::zero(),);

    let c: U256 = 23.into();
    let d: U256 = 5.into();

    assert_eq!(c % d, U256::from(3));
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", U256::from(267)), "267");
}

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", U256::from(1337)), "1337");
}
