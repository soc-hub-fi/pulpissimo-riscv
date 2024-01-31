//! mip register

// Re-expose relevant symbols from under `self::<arch>::mip::*` as `crate::register::mip::*`
#[cfg(feature = "ballast-sysctrl")]
pub use super::ballast_sysctrl::mip::*;
#[cfg(feature = "tackle")]
pub use super::tackle::mip::*;

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

// Tackle has these registers somewhere else. Defined in tackle/mie.rs
#[cfg(not(feature = "tackle"))]
read_csr_as!(Mip, 0x344);
#[cfg(not(feature = "tackle"))]
set!(0x344);
#[cfg(not(feature = "tackle"))]
clear!(0x344);
