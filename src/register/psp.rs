//! Process Stack Pointer

/// Reads the CPU register
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> u32 {
    call_asm!(__psp_r() -> u32)
}
/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> u32 {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic!(&mut r, "PSP_R");
    r
}

/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> u32 {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic!(&mut r, "PSP_R");
    r
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    call_asm!(__psp_w(bits: u32))
}
