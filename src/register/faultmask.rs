//! Fault Mask Register

#[cfg(feature = "klee-analysis")]
use klee_rs::klee_make_symbolic;
/// All exceptions are ...
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Faultmask {
    /// Active
    Active,
    /// Inactive, expect for NMI
    Inactive,
}

impl Faultmask {
    /// All exceptions are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Faultmask::Active
    }

    /// All exceptions, except for NMI, are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Faultmask::Inactive
    }
}

/// Reads the CPU register
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> Faultmask {
    let r: u32 = call_asm!(__faultmask_r() -> u32);
    if r & (1 << 0) == (1 << 0) {
        Faultmask::Inactive
    } else {
        Faultmask::Active
    }
}

/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> Faultmask {
    let mut r: u8 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic(&mut r, "FAULTMASK_R");
    if r & (1 << 0) == (1 << 0) {
        Faultmask::Inactive
    } else {
        Faultmask::Active
    }
}
