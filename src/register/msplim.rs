//! Main Stack Pointer Limit Register

/// Reads the CPU register
#[cfg(not(feature = "klee-analysis"))]
#[inline]
pub fn read() -> u32 {
    call_asm!(__msplim_r() -> u32)
}
#[cfg(feature = "klee-analysis")]
#[inline]
pub fn read() -> u32 {
    let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    klee_make_symbolic!(&mut r, "MSPLIM_R");
    r
}
/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    call_asm!(__msplim_w(bits: u32))
}
