//! Home of the `Safety` sealed type and its two variants [`Checked`] and [`Unchecked`].

/// Enforces using only checked arithmetic operations with this [`Amount`](`crate::Amount`).
///
/// This is a zero-sized enum and therefore cannot be instantiated. It can only be used in type
/// bounds.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Checked {}

/// Allows any  operations with this [`Amount`](`crate::Amount`).
///
/// This is a zero-sized enum and therefore cannot be instantiated. It can only be used in type
/// bounds.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Unchecked {}

mod sealed {
    /// When [`Unchecked`](`super::Unchecked`), the full suite of arithmetic operations is
    /// allowed, however it becomes possible to experience panics from things like
    /// division-by-zero and overflowing.
    ///
    /// When [`Checked`](`super::Checked`) is selected, only checked arithmetic operations are
    /// allowed and unchecked math becomes completely unavailable. This is ideal for
    /// environments where an uncaught panic could have dire consequences.
    pub trait Safety: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::fmt::Debug + core::hash::Hash {}
}

impl Safety for Unchecked {}
impl Safety for Checked {}

pub(crate) use sealed::Safety;
