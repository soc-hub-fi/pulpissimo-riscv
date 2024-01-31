//! mie register

// Re-expose relevant symbols from under `self::<arch>::mie::*` as `crate::register::mie::*`
#[cfg(feature = "ballast-sysctrl")]
pub use super::ballast_sysctrl::mie::*;

/// mie register
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    pub(crate) bits: usize,
}

impl Mie {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Mie, 0x304);
set!(0x304);
clear!(0x304);
