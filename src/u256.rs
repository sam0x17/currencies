use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

use num_traits::{
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, NumCast, One, PrimInt,
    Saturating, ToPrimitive, Zero,
};

// U256 with 256 bits consisting of 4 x 64-bit words
mod inner {
    use uint::construct_uint;
    construct_uint! {
        pub(crate) struct U256Inner(4);
    }
}
use inner::U256Inner;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(U256Inner);

impl NumCast for U256 {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        let Some(u_128) = n.to_u128() else {
            return None;
        };
        if let Some(i_128) = n.to_i128() {
            if i_128 < 0 {
                return None;
            }
        }
        Some(U256(U256Inner::from(u_128)))
    }
}

impl Bounded for U256 {
    fn min_value() -> Self {
        U256(U256Inner::zero())
    }

    fn max_value() -> Self {
        U256(U256Inner::max_value())
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: Self) -> Self::Output {
        U256(self.0 + rhs.0)
    }
}

impl CheckedAdd for U256 {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(&v.0).map(U256)
    }
}

impl Sub for U256 {
    type Output = U256;

    fn sub(self, rhs: Self) -> Self::Output {
        U256(self.0 - rhs.0)
    }
}

impl CheckedSub for U256 {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(&v.0).map(U256)
    }
}

impl Div for U256 {
    type Output = U256;

    fn div(self, rhs: Self) -> Self::Output {
        U256(self.0 / rhs.0)
    }
}

impl CheckedDiv for U256 {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(&v.0).map(U256)
    }
}

impl Mul for U256 {
    type Output = U256;

    fn mul(self, rhs: Self) -> Self::Output {
        U256(self.0 * rhs.0)
    }
}

impl CheckedMul for U256 {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.0.checked_mul(&v.0).map(U256)
    }
}

impl Saturating for U256 {
    fn saturating_add(self, v: Self) -> Self {
        U256(self.0.saturating_add(v.0))
    }

    fn saturating_sub(self, v: Self) -> Self {
        U256(self.0.saturating_sub(v.0))
    }
}

impl ToPrimitive for U256 {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

impl Rem for U256 {
    type Output = U256;

    fn rem(self, rhs: Self) -> Self::Output {
        U256(self.0 % rhs.0)
    }
}

impl Num for U256 {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        U256Inner::from_str_radix(str, radix).map(U256)
    }
}

impl One for U256 {
    fn one() -> Self {
        U256(U256Inner::one())
    }
}

impl Zero for U256 {
    fn zero() -> Self {
        U256(U256Inner::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Shr<usize> for U256 {
    type Output = U256;

    fn shr(self, rhs: usize) -> Self::Output {
        U256(self.0 >> rhs)
    }
}

impl Shl<usize> for U256 {
    type Output = U256;

    fn shl(self, rhs: usize) -> Self::Output {
        U256(self.0 << rhs)
    }
}

impl BitXor for U256 {
    type Output = U256;

    fn bitxor(self, rhs: Self) -> Self::Output {
        U256(self.0 ^ rhs.0)
    }
}

impl BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: Self) -> Self::Output {
        U256(self.0 & rhs.0)
    }
}

impl BitOr for U256 {
    type Output = U256;

    fn bitor(self, rhs: Self) -> Self::Output {
        U256(self.0 | rhs.0)
    }
}

impl Not for U256 {
    type Output = U256;

    fn not(self) -> Self::Output {
        U256(!self.0)
    }
}

impl PrimInt for U256 {
    fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }

    fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    fn rotate_left(self, n: u32) -> Self {
        U256(self.0.rotate_left(n))
    }

    fn rotate_right(self, n: u32) -> Self {
        U256(self.0.rotate_right(n))
    }

    fn signed_shl(self, n: u32) -> Self {
        // Unsigned integers don't have signed shift left.
        U256(self.0 << n)
    }

    fn signed_shr(self, n: u32) -> Self {
        // Unsigned integers don't have signed shift right.
        U256(self.0 >> n)
    }

    fn swap_bytes(self) -> Self {
        U256(self.0.swap_bytes())
    }

    fn from_be(x: Self) -> Self {
        U256(U256Inner::from_be(x.0))
    }

    fn from_le(x: Self) -> Self {
        U256(U256Inner::from_le(x.0))
    }

    fn to_be(self) -> Self {
        U256(self.0.to_be())
    }

    fn to_le(self) -> Self {
        U256(self.0.to_le())
    }

    fn pow(self, exp: u32) -> Self {
        U256(self.0.pow(exp))
    }

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, ParseIntError> {
        U256Inner::from_str_radix(str, radix).map(U256)
    }
}
