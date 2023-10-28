#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ForceChecked {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Unchecked {}

mod sealed {
    pub trait Safety:
        Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::fmt::Debug + core::hash::Hash
    {
    }
}

impl Safety for Unchecked {}
impl Safety for ForceChecked {}

pub(crate) use sealed::Safety;
