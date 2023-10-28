#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Checked {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Unchecked {}

mod sealed {
    pub trait Safety:
        Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::fmt::Debug + core::hash::Hash
    {
    }
}

impl Safety for Unchecked {}
impl Safety for Checked {}

pub(crate) use sealed::Safety;
