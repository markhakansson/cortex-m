//! Link register

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> u32 {
    call_asm!(__lr_r() -> u32)
}
/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> u32 {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic!(&mut r, "LR_R");
    r
}

/// For feature "klee-analysis"
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> u32 {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic!(&mut r, "LR_R");
    r
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub unsafe fn write(bits: u32) {
    call_asm!(__lr_w(bits: u32));
}
