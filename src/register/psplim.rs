//! Process Stack Pointer Limit Register

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let r;
            unsafe { llvm_asm!("mrs $0,PSPLIM" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __psplim_r() -> u32;
            }

            __psplim_r()
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => llvm_asm!("msr PSPLIM,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __psplim_w(_: u32);
            }

            __psplim_w(_bits);
        }

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
