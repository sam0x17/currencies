use core::ops::*;
use num_traits::*;

/// Wraps [`primitive_types::U256`] enhancing it with some extra trait impls needed for
/// currency manipulation.
///
/// I have submitted an issue upstream to see if we can get some additional [`num_integer`] and
/// [`num_traits`] trait impls added.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct U256(pub primitive_types::U256);

impl U256 {
    pub const MAX_VALUE: U256 = U256(primitive_types::U256::MAX);
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
