//! Interrupt / Exception context

use core::marker::PhantomData;
use core::cell::UnsafeCell;

/// Data local to a context
pub struct Local<T, Ctxt>
where
    Ctxt: Context,
{
    _ctxt: PhantomData<Ctxt>,
    data: UnsafeCell<T>,
}

impl<T, Ctxt> Local<T, Ctxt>
where
    Ctxt: Context,
{
    /// Initializes context local data
    pub const fn new(value: T) -> Self {
        Local {
            _ctxt: PhantomData,
            data: UnsafeCell::new(value),
        }
    }

    /// Acquires a reference to the context local data
    pub fn borrow<'a>(&'static self, _ctxt: &'a Ctxt) -> &'a T {
        unsafe { &*self.data.get() }
    }
}

unsafe impl<T, Ctxt> Sync for Local<T, Ctxt>
where
    Ctxt: Context,
{
}

/// A token unique to a context
pub unsafe trait Context {}
