//! Application Program Status Register

#[cfg(feature = "klee-analysis")]
use klee_rs::klee_make_symbolic;

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
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> Apsr {
    let bits: u32 = call_asm!(__apsr_r() -> u32);
    Apsr { bits }
}

/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> Apsr {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic(&mut r, "APSR_R");
    Apsr { bits: r }
}
