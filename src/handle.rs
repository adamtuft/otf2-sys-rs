use core::ops::Drop;
use std::{borrow::Borrow, mem::ManuallyDrop};

use crate::error::Status;

pub struct OwnedExternHandle<T, U>
{
    handle: *mut T,
    drop_fn: unsafe extern "C" fn(*mut T) -> U,
}

impl<T, U> std::fmt::Debug for OwnedExternHandle<T, U>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedExternHandle")
            .field("handle", &self.handle)
            .field("drop_fn", &self.drop_fn)
            .finish()
    }
}

impl<T, U> OwnedExternHandle<T, U>
// where
//     Result<(), Status>: From<E>,
{
    pub fn new(handle: *mut T, drop_fn: unsafe extern "C" fn(*mut T) -> U) -> Self {
        Self { handle, drop_fn }
    }

    pub fn dropped(&self) -> bool {
        let ptr: *const T = self.handle;
        ptr.is_null()
    }

    pub unsafe fn drop_handle_unsafe(&mut self) -> U {
        if self.dropped() {
            panic!("attempted to drop a handle that has already been dropped");
        }
        let ptr = self.handle;
        self.handle = std::ptr::null_mut();
        unsafe{ (self.drop_fn)(ptr) }
    }

    /// Get a safe shared reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    /// 
    /// # Panics
    /// 
    /// Will panic if the handle has been dropped.
    pub fn as_ref(&self) -> &T {
        if self.dropped() {
            panic!("attempted to access a dropped handle");
        }
        // SAFETY: We verified that the handle is not null.
        unsafe { &*self.handle }
    }

    /// Get a safe exclusive reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    /// 
    /// # Panics
    /// 
    /// Will panic if the handle has been dropped.
    pub fn as_mut(&mut self) -> &mut T {
        if self.dropped() {
            panic!("attempted to access a dropped handle");
        }
        // SAFETY: We verified that the handle is not null.
        unsafe { &mut *self.handle }
    }

    /// Get a raw pointer to the handle. Useful for passing to FFI functions. Doesn't enforce any
    /// lifetime semantics, so caller must ensure it is safe to hand out a `*const T`.
    /// 
    /// # Panics
    /// 
    /// Will panic if the handle has been dropped.
    pub unsafe fn as_raw(&self) -> *const T {
        if self.dropped() {
            panic!("attempted to access a dropped handle");
        }
        // SAFETY: We verified that the handle is not null.
        self.handle as _
    }

    /// Get a raw pointer to the handle. Useful for passing to FFI functions. Doesn't enforce any
    /// lifetime semantics, caller so must ensure it is safe to hand out a `*mut T`.
    /// 
    /// # Panics
    /// 
    /// Will panic if the handle has been dropped.
    pub unsafe fn as_raw_mut(&mut self) -> *mut T {
        if self.dropped() {
            panic!("attempted to access a dropped handle");
        }
        // SAFETY: We verified that the handle is not null.
        unsafe { self.as_raw() as _ }
    }
}

impl<T, E> Drop for OwnedExternHandle<T, E> {
    fn drop(&mut self) {
        if ! self.dropped() {
            #[cfg(test)]
            {
                eprintln!("Dropping handle:");
                dbg!(&self);
            }
            // SAFETY: We verified that the handle is not null.
            unsafe { self.drop_handle_unsafe() };
        }
    }
}

impl<T, U> OwnedExternHandle<T, U>
where
    Result<(), Status>: From<U>,
{
    pub fn drop_handle(&mut self) -> Result<(), Status> {
        if self.dropped() {
            Ok(())
        } else {
            #[cfg(test)]
            {
                eprintln!("Dropping handle:");
                dbg!(&self);
            }
            let ptr = self.handle;
            self.handle = std::ptr::null_mut();
            unsafe { (self.drop_fn)(ptr) }.into()
        }
    }
}
