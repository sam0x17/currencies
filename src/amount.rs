use core::marker::PhantomData;
use num_traits::PrimInt;

use crate::currency::*;
use uint::construct_uint;

pub trait Base: PrimInt {}
impl<T: PrimInt> Base for T {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount<
    // Represents the underlying (signed or un-signed) primitive integer type used to represent
    // this amount
    B: Base = u64,
    // Determines the number of supported fractional decimal digits for this amount (a value of
    // 0 means only integers can be represented). This is also the position of the decimal
    // point from the RHS of the underlying integer representation of the amount.
    const FRAC_DIGITS: usize = 2,
    // When set to true, only checked operations are allowed on this amount, making it
    // impossible to have unhandled underflow/overflow errors with this amount
    const SAFE: bool = true,
    C: Currency = USD,
>(B, PhantomData<C>);

impl<B: Base, const FRAC_DIGITS: usize, const SAFE: bool, C: Currency>
    Amount<B, FRAC_DIGITS, SAFE, C>
{
    pub const fn from_raw(amount: B) -> Self {
        Amount(amount, PhantomData)
    }
}

#[test]
fn test_types() {
    let a: Amount = Amount::from_raw(1000_00);
    let b: Amount = Amount::from_raw(200_00);
    assert!(a != b);
    assert!(a == a);
}
