use core::marker::PhantomData;
use num_traits::PrimInt;

use crate::currency::*;

pub struct Amount<B: PrimInt = u64, const P: usize = 2, C: Currency = USD>(B, PhantomData<C>);
