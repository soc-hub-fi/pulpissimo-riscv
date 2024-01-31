//! mip register

// Re-expose relevant symbols from under `self::<arch>::mip::*` as `crate::register::mip::*`
#[cfg(feature = "ballast-sysctrl")]
pub use super::ballast_sysctrl::mip::*;

/// mip register
#[derive(Clone, Copy, Debug)]
pub struct Mip {
    pub(crate) bits: usize,
}

impl Mip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Mip, 0x344);
set!(0x344);
clear!(0x344);
