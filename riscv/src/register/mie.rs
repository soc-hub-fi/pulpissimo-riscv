//! mie register

// Re-expose relevant symbols from under `self::<arch>::mie::*` as `crate::register::mie::*`
#[cfg(feature = "ballast-sysctrl")]
pub use super::ballast_sysctrl::mie::*;
#[cfg(feature = "tackle")]
pub use super::tackle::mie::*;

/// mie register
///
/// ## Tackle-specific
///
/// MIE has all bits up (0xffff_ffff) on boot on Tackle unlike other RISC-V.
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

// Tackle has these registers somewhere else. Defined in tackle/mie.rs
#[cfg(not(feature = "tackle"))]
read_csr_as!(Mie, 0x304);
#[cfg(not(feature = "tackle"))]
set!(0x304);
#[cfg(not(feature = "tackle"))]
clear!(0x304);
