//! Application Program Status Register

#[cfg(cortex_m)]
use core::arch::asm;

/// Application Program Status Register
#[derive(Clone, Copy, Debug)]
pub struct Apsr {
    bits: u32,
}

impl Apsr {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// DSP overflow and saturation flag
    #[inline]
    pub fn q(self) -> bool {
        self.bits & (1 << 27) == (1 << 27)
    }

    /// Overflow flag
    #[inline]
    pub fn v(self) -> bool {
        self.bits & (1 << 28) == (1 << 28)
    }

    /// Carry or borrow flag
    #[inline]
    pub fn c(self) -> bool {
        self.bits & (1 << 29) == (1 << 29)
    }

    /// Zero flag
    #[inline]
    pub fn z(self) -> bool {
        self.bits & (1 << 30) == (1 << 30)
    }

    /// Negative flag
    #[inline]
    pub fn n(self) -> bool {
        self.bits & (1 << 31) == (1 << 31)
    }
}

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> Apsr {
    let bits;
    unsafe { asm!("mrs {}, APSR", out(reg) bits, options(nomem, nostack, preserves_flags)) };
    Apsr { bits }
}
