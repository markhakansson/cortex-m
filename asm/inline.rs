//! Inline assembly implementing the routines exposed in `cortex_m::asm`.
//!
//! If the `inline-asm` feature is enabled, these functions will be directly called by the
//! `cortex-m` wrappers. Otherwise, `cortex-m` links against them via prebuilt archives.
//!
//! All of these functions should be blanket-`unsafe`. `cortex-m` provides safe wrappers where
//! applicable.

use core::sync::atomic::{compiler_fence, Ordering};

#[inline(always)]
pub unsafe fn __bkpt(imm: u8) {
    macro_rules! call {
        ($imm:expr) => {
            asm!(concat!("bkpt ", stringify!($imm)))
        };
    }
    #[allow(unused)]
    macro_rules! constify_imm8 {
        ($imm8:expr, $expand:ident) => {
            #[allow(overflowing_literals)]
            match ($imm8) & 0b1111_1111 {
                0 => $expand!(0),
                1 => $expand!(1),
                2 => $expand!(2),
                3 => $expand!(3),
                4 => $expand!(4),
                5 => $expand!(5),
                6 => $expand!(6),
                7 => $expand!(7),
                8 => $expand!(8),
                9 => $expand!(9),
                10 => $expand!(10),
                11 => $expand!(11),
                12 => $expand!(12),
                13 => $expand!(13),
                14 => $expand!(14),
                15 => $expand!(15),
                16 => $expand!(16),
                17 => $expand!(17),
                18 => $expand!(18),
                19 => $expand!(19),
                20 => $expand!(20),
                21 => $expand!(21),
                22 => $expand!(22),
                23 => $expand!(23),
                24 => $expand!(24),
                25 => $expand!(25),
                26 => $expand!(26),
                27 => $expand!(27),
                28 => $expand!(28),
                29 => $expand!(29),
                30 => $expand!(30),
                31 => $expand!(31),
                32 => $expand!(32),
                33 => $expand!(33),
                34 => $expand!(34),
                35 => $expand!(35),
                36 => $expand!(36),
                37 => $expand!(37),
                38 => $expand!(38),
                39 => $expand!(39),
                40 => $expand!(40),
                41 => $expand!(41),
                42 => $expand!(42),
                43 => $expand!(43),
                44 => $expand!(44),
                45 => $expand!(45),
                46 => $expand!(46),
                47 => $expand!(47),
                48 => $expand!(48),
                49 => $expand!(49),
                50 => $expand!(50),
                51 => $expand!(51),
                52 => $expand!(52),
                53 => $expand!(53),
                54 => $expand!(54),
                55 => $expand!(55),
                56 => $expand!(56),
                57 => $expand!(57),
                58 => $expand!(58),
                59 => $expand!(59),
                60 => $expand!(60),
                61 => $expand!(61),
                62 => $expand!(62),
                63 => $expand!(63),
                64 => $expand!(64),
                65 => $expand!(65),
                66 => $expand!(66),
                67 => $expand!(67),
                68 => $expand!(68),
                69 => $expand!(69),
                70 => $expand!(70),
                71 => $expand!(71),
                72 => $expand!(72),
                73 => $expand!(73),
                74 => $expand!(74),
                75 => $expand!(75),
                76 => $expand!(76),
                77 => $expand!(77),
                78 => $expand!(78),
                79 => $expand!(79),
                80 => $expand!(80),
                81 => $expand!(81),
                82 => $expand!(82),
                83 => $expand!(83),
                84 => $expand!(84),
                85 => $expand!(85),
                86 => $expand!(86),
                87 => $expand!(87),
                88 => $expand!(88),
                89 => $expand!(89),
                90 => $expand!(90),
                91 => $expand!(91),
                92 => $expand!(92),
                93 => $expand!(93),
                94 => $expand!(94),
                95 => $expand!(95),
                96 => $expand!(96),
                97 => $expand!(97),
                98 => $expand!(98),
                99 => $expand!(99),
                100 => $expand!(100),
                101 => $expand!(101),
                102 => $expand!(102),
                103 => $expand!(103),
                104 => $expand!(104),
                105 => $expand!(105),
                106 => $expand!(106),
                107 => $expand!(107),
                108 => $expand!(108),
                109 => $expand!(109),
                110 => $expand!(110),
                111 => $expand!(111),
                112 => $expand!(112),
                113 => $expand!(113),
                114 => $expand!(114),
                115 => $expand!(115),
                116 => $expand!(116),
                117 => $expand!(117),
                118 => $expand!(118),
                119 => $expand!(119),
                120 => $expand!(120),
                121 => $expand!(121),
                122 => $expand!(122),
                123 => $expand!(123),
                124 => $expand!(124),
                125 => $expand!(125),
                126 => $expand!(126),
                127 => $expand!(127),
                128 => $expand!(128),
                129 => $expand!(129),
                130 => $expand!(130),
                131 => $expand!(131),
                132 => $expand!(132),
                133 => $expand!(133),
                134 => $expand!(134),
                135 => $expand!(135),
                136 => $expand!(136),
                137 => $expand!(137),
                138 => $expand!(138),
                139 => $expand!(139),
                140 => $expand!(140),
                141 => $expand!(141),
                142 => $expand!(142),
                143 => $expand!(143),
                144 => $expand!(144),
                145 => $expand!(145),
                146 => $expand!(146),
                147 => $expand!(147),
                148 => $expand!(148),
                149 => $expand!(149),
                150 => $expand!(150),
                151 => $expand!(151),
                152 => $expand!(152),
                153 => $expand!(153),
                154 => $expand!(154),
                155 => $expand!(155),
                156 => $expand!(156),
                157 => $expand!(157),
                158 => $expand!(158),
                159 => $expand!(159),
                160 => $expand!(160),
                161 => $expand!(161),
                162 => $expand!(162),
                163 => $expand!(163),
                164 => $expand!(164),
                165 => $expand!(165),
                166 => $expand!(166),
                167 => $expand!(167),
                168 => $expand!(168),
                169 => $expand!(169),
                170 => $expand!(170),
                171 => $expand!(171),
                172 => $expand!(172),
                173 => $expand!(173),
                174 => $expand!(174),
                175 => $expand!(175),
                176 => $expand!(176),
                177 => $expand!(177),
                178 => $expand!(178),
                179 => $expand!(179),
                180 => $expand!(180),
                181 => $expand!(181),
                182 => $expand!(182),
                183 => $expand!(183),
                184 => $expand!(184),
                185 => $expand!(185),
                186 => $expand!(186),
                187 => $expand!(187),
                188 => $expand!(188),
                189 => $expand!(189),
                190 => $expand!(190),
                191 => $expand!(191),
                192 => $expand!(192),
                193 => $expand!(193),
                194 => $expand!(194),
                195 => $expand!(195),
                196 => $expand!(196),
                197 => $expand!(197),
                198 => $expand!(198),
                199 => $expand!(199),
                200 => $expand!(200),
                201 => $expand!(201),
                202 => $expand!(202),
                203 => $expand!(203),
                204 => $expand!(204),
                205 => $expand!(205),
                206 => $expand!(206),
                207 => $expand!(207),
                208 => $expand!(208),
                209 => $expand!(209),
                210 => $expand!(210),
                211 => $expand!(211),
                212 => $expand!(212),
                213 => $expand!(213),
                214 => $expand!(214),
                215 => $expand!(215),
                216 => $expand!(216),
                217 => $expand!(217),
                218 => $expand!(218),
                219 => $expand!(219),
                220 => $expand!(220),
                221 => $expand!(221),
                222 => $expand!(222),
                223 => $expand!(223),
                224 => $expand!(224),
                225 => $expand!(225),
                226 => $expand!(226),
                227 => $expand!(227),
                228 => $expand!(228),
                229 => $expand!(229),
                230 => $expand!(230),
                231 => $expand!(231),
                232 => $expand!(232),
                233 => $expand!(233),
                234 => $expand!(234),
                235 => $expand!(235),
                236 => $expand!(236),
                237 => $expand!(237),
                238 => $expand!(238),
                239 => $expand!(239),
                240 => $expand!(240),
                241 => $expand!(241),
                242 => $expand!(242),
                243 => $expand!(243),
                244 => $expand!(244),
                245 => $expand!(245),
                246 => $expand!(246),
                247 => $expand!(247),
                248 => $expand!(248),
                249 => $expand!(249),
                250 => $expand!(250),
                251 => $expand!(251),
                252 => $expand!(252),
                253 => $expand!(253),
                254 => $expand!(254),
                _ => $expand!(255),
            }
        };
    }
    constify_imm8!(imm, call);
}

#[inline(always)]
pub unsafe fn __control_r() -> u32 {
    let r;
    asm!("mrs {}, CONTROL", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __control_w(w: u32) {
    // ISB is required after writing to CONTROL,
    // per ARM architectural requirements (see Application Note 321).
    asm!(
        "msr CONTROL, {}",
        "isb",
        in(reg) w
    );

    // Ensure memory accesses are not reordered around the CONTROL update.
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __cpsid() {
    asm!("cpsid i");

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __cpsie() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    asm!("cpsie i");
}

#[inline(always)]
pub unsafe fn __delay(cyc: u32) {
    // The loop will normally take 3 to 4 CPU cycles per iteration, but superscalar cores
    // (eg. Cortex-M7) can potentially do it in 2, so we use that as the lower bound, since delaying
    // for more cycles is okay.
    // Add 1 to prevent an integer underflow which would cause a long freeze
    let real_cyc = 1 + cyc / 2;
    asm!(
        // Use local labels to avoid R_ARM_THM_JUMP8 relocations which fail on thumbv6m.
        "1:",
        "subs {}, #1",
        "bne 1b",
        inout(reg) real_cyc => _
    );
}

#[inline(always)]
pub unsafe fn __dmb() {
    compiler_fence(Ordering::SeqCst);
    asm!("dmb");
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __dsb() {
    compiler_fence(Ordering::SeqCst);
    asm!("dsb");
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __isb() {
    compiler_fence(Ordering::SeqCst);
    asm!("isb");
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn __msp_r() -> u32 {
    let r;
    asm!("mrs {}, MSP", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __msp_w(val: u32) {
    asm!("msr MSP, {}", in(reg) val);
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __apsr_r() -> u32 {
    let r;
    asm!("mrs {}, APSR", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __nop() {
    // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
    // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
    // nops when they call `nop` N times, let's not add that option.
    asm!("nop");
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __pc_r() -> u32 {
    let r;
    asm!("mov {}, pc", out(reg) r);
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __pc_w(val: u32) {
    asm!("mov pc, {}", in(reg) val);
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __lr_r() -> u32 {
    let r;
    asm!("mov {}, lr", out(reg) r);
    r
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub unsafe fn __lr_w(val: u32) {
    asm!("mov lr, {}", in(reg) val);
}

#[inline(always)]
pub unsafe fn __primask_r() -> u32 {
    let r;
    asm!("mrs {}, PRIMASK", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __psp_r() -> u32 {
    let r;
    asm!("mrs {}, PSP", out(reg) r);
    r
}

#[inline(always)]
pub unsafe fn __psp_w(val: u32) {
    asm!("msr PSP, {}", in(reg) val);
}

#[inline(always)]
pub unsafe fn __sev() {
    asm!("sev");
}

#[inline(always)]
pub unsafe fn __udf() -> ! {
    asm!("udf #0", options(noreturn));
}

#[inline(always)]
pub unsafe fn __wfe() {
    asm!("wfe");
}

#[inline(always)]
pub unsafe fn __wfi() {
    asm!("wfi");
}

/// Semihosting syscall.
#[inline(always)]
pub unsafe fn __sh_syscall(mut nr: u32, arg: u32) -> u32 {
    asm!("bkpt #0xab", inout("r0") nr, in("r1") arg);
    nr
}

/// Set CONTROL.SPSEL to 0, write `msp` to MSP, branch to `rv`.
#[inline(always)]
pub unsafe fn __bootstrap(msp: u32, rv: u32) -> ! {
    asm!(
        "mrs {tmp}, CONTROL",
        "bics {tmp}, {spsel}",
        "msr CONTROL, {tmp}",
        "isb",
        "msr MSP, {msp}",
        "bx {rv}",
        // `out(reg) _` is not permitted in a `noreturn` asm! call,
        // so instead use `in(reg) 0` and don't restore it afterwards.
        tmp = in(reg) 0,
        spsel = in(reg) 2,
        msp = in(reg) msp,
        rv = in(reg) rv,
        options(noreturn),
    );
}

// v7m *AND* v8m.main, but *NOT* v8m.base
#[cfg(any(armv7m, armv8m_main))]
pub use self::v7m::*;
#[cfg(any(armv7m, armv8m_main))]
mod v7m {
    use core::sync::atomic::{compiler_fence, Ordering};

    #[inline(always)]
    pub unsafe fn __basepri_max(val: u8) {
        asm!("msr BASEPRI_MAX, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __basepri_r() -> u8 {
        let r;
        asm!("mrs {}, BASEPRI", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __basepri_w(val: u8) {
        asm!("msr BASEPRI, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __faultmask_r() -> u32 {
        let r;
        asm!("mrs {}, FAULTMASK", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __enable_icache() {
        asm!(
            "ldr {0}, =0xE000ED14",         // CCR
            "mrs {2}, PRIMASK",             // save critical nesting info
            "cpsid i",                      // mask interrupts
            "ldr {1}, [{0}]",               // read CCR
            "orr.w {1}, {1}, #(1 << 17)",   // Set bit 17, IC
            "str {1}, [{0}]",               // write it back
            "dsb",                          // ensure store completes
            "isb",                          // synchronize pipeline
            "msr PRIMASK, {2}",             // unnest critical section
            out(reg) _,
            out(reg) _,
            out(reg) _,
        );
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub unsafe fn __enable_dcache() {
        asm!(
            "ldr {0}, =0xE000ED14",         // CCR
            "mrs {2}, PRIMASK",             // save critical nesting info
            "cpsid i",                      // mask interrupts
            "ldr {1}, [{0}]",               // read CCR
            "orr.w {1}, {1}, #(1 << 16)",   // Set bit 16, DC
            "str {1}, [{0}]",               // write it back
            "dsb",                          // ensure store completes
            "isb",                          // synchronize pipeline
            "msr PRIMASK, {2}",             // unnest critical section
            out(reg) _,
            out(reg) _,
            out(reg) _,
        );
        compiler_fence(Ordering::SeqCst);
    }
}

#[cfg(armv7em)]
pub use self::v7em::*;
#[cfg(armv7em)]
mod v7em {
    #[inline(always)]
    pub unsafe fn __basepri_max_cm7_r0p1(val: u8) {
        asm!(
            "mrs {1}, PRIMASK",
            "cpsid i",
            "tst.w {1}, #1",
            "msr BASEPRI_MAX, {0}",
            "it ne",
            "bxne lr",
            "cpsie i",
            in(reg) val,
            out(reg) _,
        );
    }

    #[inline(always)]
    pub unsafe fn __basepri_w_cm7_r0p1(val: u8) {
        asm!(
            "mrs {1}, PRIMASK",
            "cpsid i",
            "tst.w {1}, #1",
            "msr BASEPRI, {0}",
            "it ne",
            "bxne lr",
            "cpsie i",
            in(reg) val,
            out(reg) _,
        );
    }
}

#[cfg(armv8m)]
pub use self::v8m::*;
/// Baseline and Mainline.
#[cfg(armv8m)]
mod v8m {
    #[inline(always)]
    pub unsafe fn __tt(mut target: u32) -> u32 {
        asm!("tt {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __ttt(mut target: u32) -> u32 {
        asm!("ttt {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __tta(mut target: u32) -> u32 {
        asm!("tta {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __ttat(mut target: u32) -> u32 {
        asm!("ttat {target}, {target}", target = inout(reg) target);
        target
    }

    #[inline(always)]
    pub unsafe fn __msp_ns_r() -> u32 {
        let r;
        asm!("mrs {}, MSP_NS", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __msp_ns_w(val: u32) {
        asm!("msr MSP_NS, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __bxns(val: u32) {
        asm!("BXNS {}", in(reg) val);
    }
}

#[cfg(armv8m_main)]
pub use self::v8m_main::*;
/// Mainline only.
#[cfg(armv8m_main)]
mod v8m_main {
    #[inline(always)]
    pub unsafe fn __msplim_r() -> u32 {
        let r;
        asm!("mrs {}, MSPLIM", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __msplim_w(val: u32) {
        asm!("msr MSPLIM, {}", in(reg) val);
    }

    #[inline(always)]
    pub unsafe fn __psplim_r() -> u32 {
        let r;
        asm!("mrs {}, PSPLIM", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __psplim_w(val: u32) {
        asm!("msr PSPLIM, {}", in(reg) val);
    }
}

#[cfg(has_fpu)]
pub use self::fpu::*;
/// All targets with FPU.
#[cfg(has_fpu)]
mod fpu {
    #[inline(always)]
    pub unsafe fn __fpscr_r() -> u32 {
        let r;
        asm!("vmrs {}, fpscr", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn __fpscr_w(val: u32) {
        asm!("vmsr fpscr, {}", in(reg) val);
    }
}
