//! mtvec register

// Re-expose relevant symbols from under `self::<arch>::mtvec::*` as `crate::register::mtvec::*`
#[cfg(feature = "tackle")]
pub use super::tackle::mtvec::*;

/// mtvec register
#[derive(Clone, Copy, Debug)]
pub struct Mtvec {
    pub(crate) bits: usize,
}

/// Trap mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Mtvec {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    #[inline]
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    #[inline]
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            _ => None,
        }
    }
}

#[cfg(not(feature = "tackle"))]
read_csr_as!(Mtvec, 0x305);
#[cfg(not(feature = "tackle"))]
write_csr!(0x305);

/// Writes the CSR
#[inline]
#[cfg(not(feature = "tackle"))]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits = addr + mode as usize;
    _write(bits);
}
