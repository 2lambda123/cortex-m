//! Miscellaneous assembly instructions

// When inline assembly is enabled, pull in the assembly routines here. `call_asm!` will invoke
// these routines.
#[cfg(feature = "inline-asm")]
#[path = "../asm/inline.rs"]
pub(crate) mod inline;

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
pub fn bkpt() {
    call_asm!(__bkpt());
}

/// Blocks the program for *at least* `n` instruction cycles
///
/// This is implemented in assembly so its execution time is independent of the optimization
/// level, however it is dependent on the specific architecture and core configuration.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution
/// and the execution time may vary with other factors. This delay is mainly useful for simple
/// timer-less initialization of peripherals if and only if accurate timing is not essential. In
/// any other case please use a more accurate method to produce a delay.
#[inline]
pub fn delay(n: u32) {
    // NOTE(divide by 4) is easier to compute than `/ 3` because it's just a shift (`>> 2`).
    let real_cyc = n / 4 + 1;
    call_asm!(__delay(real_cyc: u32));
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    call_asm!(__nop());
}

/// Generate an Undefined Instruction exception.
///
/// Can be used as a stable alternative to `core::intrinsics::abort`.
#[inline]
pub fn udf() -> ! {
    call_asm!(__udf() -> !)
}

/// Wait For Event
#[inline]
pub fn wfe() {
    call_asm!(__wfe())
}

/// Wait For Interrupt
#[inline]
pub fn wfi() {
    call_asm!(__wfi())
}

/// Send Event
#[inline]
pub fn sev() {
    call_asm!(__sev())
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
    call_asm!(__isb())
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
pub fn dsb() {
    call_asm!(__dsb())
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
    call_asm!(__dmb())
}

/// Test Target
///
/// Queries the Security state and access permissions of a memory location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __tt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tt(addr: *mut u32) -> u32 {
    let addr = addr as u32;
    call_asm!(__tt(addr: u32) -> u32)
}

/// Test Target Unprivileged
///
/// Queries the Security state and access permissions of a memory location for an unprivileged
/// access to that location.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __ttt function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttt(addr: *mut u32) -> u32 {
    let addr = addr as u32;
    call_asm!(__ttt(addr: u32) -> u32)
}

/// Test Target Alternate Domain
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure access
/// to that location. This instruction is only valid when executing in Secure state and is
/// undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __tta function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn tta(addr: *mut u32) -> u32 {
    let addr = addr as u32;
    call_asm!(__tta(addr: u32) -> u32)
}

/// Test Target Alternate Domain Unprivileged
///
/// Queries the Security state and access permissions of a memory location for a Non-Secure and
/// unprivileged access to that location. This instruction is only valid when executing in Secure
/// state and is undefined if used from Non-Secure state.
/// Returns a Test Target Response Payload (cf section D1.2.215 of
/// Armv8-M Architecture Reference Manual).
#[inline]
#[cfg(armv8m)]
// The __ttat function does not dereference the pointer received.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ttat(addr: *mut u32) -> u32 {
    let addr = addr as u32;
    call_asm!(__ttat(addr: u32) -> u32)
}
