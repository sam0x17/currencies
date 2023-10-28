use num_traits::PrimInt;

pub struct Amount<B: PrimInt = u64, const P: usize = 2>(B);
