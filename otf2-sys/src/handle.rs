use core::ops::Drop;
use std::{borrow::Borrow, mem::ManuallyDrop};

use crate::error::Status;

pub struct Handle<T, U>
{
    handle: *mut T,
    drop_fn: fn(&mut T) -> U,
    _marker: std::marker::PhantomData<T>,
    owned_type: &'static str,
}

impl<T, U> std::fmt::Debug for Handle<T, U>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("owned_type", &self.owned_type)
            .field("owned_value", &self.as_ref())
            .field("handle", &self.handle)
            .field("drop_fn", &self.drop_fn)
            .finish()
    }
}

impl<T, U> Handle<T, U>
{
    /// Create a new handle from a raw pointer and a drop function. If the pointer is null, returns
    /// `None`.
    /// 
    /// # Visibility
    /// 
    /// This is `pub(crate)` because it can only be created by calling FFI functions. Consumers
    /// of the crate shouldn't create handles directly, instead specialisations of this type
    /// should define constructors that manage creation of handles.
    pub(crate) fn new(handle: *mut T, drop_fn: fn(&mut T) -> U) -> Option<Self> {
        if handle.is_null() {
            None
        } else {
            Some(Self { handle, drop_fn, _marker: std::marker::PhantomData, owned_type: std::any::type_name::<T>()})
        }
    }

    pub fn dropped(&self) -> bool {
        self.handle.is_null()
    }

    fn panic_if_dropped(&self) {
        if self.dropped() {
            panic!("attempted to access a dropped handle");
        }
    }

    /// Call the drop function for the handle, returning the result. Panics if the handle has
    /// already been dropped.
    /// 
    /// # Panics
    /// 
    /// Will panic if the handle has already been dropped.
    pub fn drop_handle(&mut self) -> U {
        self.panic_if_dropped();
        let result = (self.drop_fn)(self.as_mut());
        self.handle = std::ptr::null_mut();
        result
    }

    /// Get a safe shared reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    pub fn as_ref(&self) -> &T {
        self.panic_if_dropped();
        unsafe { &*self.handle }
    }

    /// Get a safe exclusive reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    pub fn as_mut(&mut self) -> &mut T {
        self.panic_if_dropped();
        unsafe { &mut *self.handle }
    }

    /// Get a raw pointer to the handle. Useful for passing to FFI functions. Doesn't enforce any
    /// lifetime semantics, so caller must ensure it is safe to hand out a `*const T`.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    /// 
    /// # Visibility
    /// 
    /// This function is marked as `pub(crate)` because it is necessary for calling low-level FFI
    /// functions. It should not be exposed to consumers of the crate.
    pub(crate) fn as_raw(&self) -> *const T {
        self.panic_if_dropped();
        self.handle as _
    }

    /// Get a raw pointer to the handle. Useful for passing to FFI functions. Doesn't enforce any
    /// lifetime semantics, caller so must ensure it is safe to hand out a `*mut T`.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    /// 
    /// # Visibility
    /// 
    /// This function is marked as `pub(crate)` because it is necessary for calling low-level FFI
    /// functions. It should not be exposed to consumers of the crate.
    pub(crate) fn as_raw_mut(&mut self) -> *mut T {
        self.panic_if_dropped();
        self.as_raw() as _
    }
}

impl<T, E> Drop for Handle<T, E> {
    fn drop(&mut self) {
        if !self.dropped() {
            #[cfg(test)]
            {
                eprintln!("Dropping handle to {} at {:?}", self.owned_type, self.handle);
            }
            // SAFETY: We verified that the handle is not null.
            self.drop_handle();
        }
    }
}

impl<T, U> Into<*mut T> for Handle<T, U> {
    fn into(self) -> *mut T {
        // SAFETY: Ownership is being transferred to the caller, so the handle should not be
        // dropped after the owned pointer is moved out.
        let mut this = ManuallyDrop::new(self);
        this.as_raw_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let handle = Handle::new(Box::into_raw(Box::new(42)), |ptr| {
            // To drop from the current handle, transfer ownership to a Box which will deallocate
            // when dropped.
            unsafe { Box::from_raw(ptr) }
        });
        dbg!(&handle);
        assert!(handle.is_some());
        let mut handle = handle.unwrap();
        assert_eq!(*handle.as_ref(), 42);
        assert_eq!(unsafe { *handle.as_raw() }, 42);
        assert_eq!(unsafe { *handle.as_raw_mut() }, 42);
        assert!(!handle.dropped());
        let dropped_value = handle.drop_handle();
        assert!(handle.dropped());
        assert_eq!(*dropped_value, 42);
        let other_handle = Handle::new(std::ptr::null_mut(), |_ptr: &mut bool| {
            unreachable!("This should not be called");
        });
        assert!(other_handle.is_none());
        let good_handle = Handle::new(Box::into_raw(Box::new(100)), |ptr| {
            unsafe { Box::from_raw(ptr) }
        });
        dbg!(&good_handle);
        assert!(good_handle.is_some());
        let good_handle = Handle::new(Box::into_raw(Box::new(false)), |ptr| {
            unsafe { Box::from_raw(ptr) }
        });
        dbg!(&good_handle);
        assert!(good_handle.is_some());
        let good_handle = Handle::new(Box::into_raw(Box::new(vec![false, true, false])), |ptr| {
            unsafe { Box::from_raw(ptr) }
        });
        dbg!(&good_handle);
        assert!(good_handle.is_some());
        let good_handle = Handle::new(Box::into_raw(Box::new(vec!["hello", "world!"])), |ptr| {
            unsafe { Box::from_raw(ptr) }
        });
        dbg!(&good_handle);
        assert!(good_handle.is_some());
    }
}
