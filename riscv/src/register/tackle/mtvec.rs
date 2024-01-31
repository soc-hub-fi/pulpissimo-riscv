use crate::register::mtvec::{Mtvec, TrapMode};

// Note that while these are usually at 0x305 on RISC-V (including on Ballast
// cores), Tackle's SysCtrl places it at 0x7d1.
read_csr_as!(Mtvec, 0x7d1);
write_csr!(0x7d1);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits = addr + mode as usize;
    _write(bits);
}
