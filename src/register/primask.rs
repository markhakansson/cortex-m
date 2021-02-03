//! Priority mask register

/// All exceptions with configurable priority are ...
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl Primask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Primask::Inactive
    }
}

/// Reads the CPU register
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> Primask {
    let r: u32 = call_asm!(__primask_r() -> u32);
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}

/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> Primask {
    fn read_raw() -> u32 {
        let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        klee_make_symbolic!(&mut r, "PRIMASK_R");
        r
    }

    let r = read_raw();
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}

#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> Primask {
    fn read_raw() -> u32 {
        let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        klee_make_symbolic!(&mut r, "PRIMASK_R");
        r
    }

    let r = read_raw();
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}
