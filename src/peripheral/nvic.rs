//! Nested Vector Interrupt Controller

use interrupt::Nr;
use volatile_register::{RO, RW};

/// Registers
#[repr(C)]
pub struct Registers {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 8],
    reserved0: [u32; 24],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 8],
    reserved1: [u32; 24],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 8],
    reserved2: [u32; 24],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 8],
    reserved3: [u32; 24],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 8],
    reserved4: [u32; 56],
    /// Interrupt Priority
    pub ipr: [RW<u8>; 240],
}

impl Registers {
    /// Clears `interrupt` pending state
    pub fn clear_pending<I>(&mut self, interrupt: I)
        where I: Nr
    {
        let nr = interrupt.nr();

        self.icpr[usize::from(nr / 32)].write(1 << (nr % 32));
    }

    /// Disables `interrupt`
    pub fn disable<I>(&mut self, interrupt: I)
        where I: Nr
    {
        let nr = interrupt.nr();

        self.icer[usize::from(nr / 32)].write(1 << (nr % 32));
    }

    /// Enables `interrupt`
    pub fn enable<I>(&mut self, interrupt: I)
        where I: Nr
    {
        let nr = interrupt.nr();

        self.iser[usize::from(nr / 32)].write(1 << (nr % 32));
    }

    /// Gets the priority of `interrupt`
    pub fn get_priority<I>(&mut self, interrupt: I) -> u8
        where I: Nr
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr / 4)].read()
    }

    /// Is `interrupt` active or pre-empted and stacked
    pub fn is_active<I>(&self, interrupt: I) -> bool
        where I: Nr
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iabr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is enabled
    pub fn is_enabled<I>(&self, interrupt: I) -> bool
        where I: Nr
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iser[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is pending
    pub fn is_pending<I>(&self, interrupt: I) -> bool
        where I: Nr
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.ispr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Forces `interrupt` into pending state
    pub fn set_pending<I>(&mut self, interrupt: I)
        where I: Nr
    {
        let nr = interrupt.nr();

        self.ispr[usize::from(nr / 32)].write(1 << (nr % 32));
    }

    /// Sets the priority of `interrupt` to `prio`
    pub fn set_priority<I>(&mut self, interrupt: I, prio: u8)
        where I: Nr
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr / 4)].write(prio);
    }
}
