//! Defines a thin wrapper around a raw pointer which can be used to ensure safe access
//! to the underlying data. Is not concerned with allocation or clean-up of the pointer.

pub struct Handle<T> {
    handle: *mut T,
    _marker: std::marker::PhantomData<T>,
    owned_type: &'static str,
}

impl<T> std::fmt::Debug for Handle<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle<{}> at {:?}", self.owned_type, self.handle)
    }
}

impl<T> Handle<T> {
    /// Create a new handle from a raw pointer. If the pointer is null, returns
    /// `None`.
    /// 
    /// # Visibility
    /// 
    /// This is `pub(crate)` because it can only be created by calling FFI functions. Consumers
    /// of the crate shouldn't create handles directly, instead specialisations of this type
    /// should define constructors and destructors that manage creation and destruction of handles.
    pub(crate) fn from_raw(handle: *mut T) -> Option<Self> {
        if handle.is_null() {
            None
        } else {
            Some(Handle::from_raw_unchecked(handle))
        }
    }

    /// WARNING: only use this if you *know* the given pointer is valid!!!
    pub(crate) fn from_raw_unchecked(handle: *mut T) -> Self {
        let typename = std::any::type_name::<T>();
        let owned_type = typename.strip_suffix("_struct")
            .unwrap_or(typename);
        let owned_type = owned_type.rsplit_once("::")
            .map_or(owned_type, |(_, name)| name);
        Self { handle, _marker: std::marker::PhantomData, owned_type }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    fn panic_if_null(&self) {
        if self.is_null() {
            panic!("attempted to access a null handle");
        }
    }

    /// Nullify the handle, returning a raw pointer to the underlying value.
    /// 
    pub fn take(&mut self) -> *mut T {
        let ptr = self.handle;
        self.handle = std::ptr::null_mut();
        ptr
    }

    /// Get a safe shared reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    pub fn as_ref(&self) -> &T {
        self.panic_if_null();
        unsafe { &*self.handle }
    }

    /// Get a safe exclusive reference to the handle. Useful to allow enforcing lifetime semantics
    /// for the underlying pointer.
    ///
    /// # Panics
    ///
    /// Will panic if the handle has been dropped.
    pub fn as_mut(&mut self) -> &mut T {
        self.panic_if_null();
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
    pub(crate) fn as_ptr(&self) -> *const T {
        self.panic_if_null();
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
    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.panic_if_null();
        self.as_ptr() as _
    }
}

impl<T> From<Handle<T>> for *mut T {
    fn from(mut val: Handle<T>) -> Self {
        val.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
    struct NumberHandle(Handle<i32>);

    impl NumberHandle {
        fn new(value: i32) -> Self {
            let handle = Handle::from_raw(Box::into_raw(Box::new(value))).expect("Failed to create handle");
            Self(handle)
        }
    }

    impl Drop for NumberHandle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { drop(Box::from_raw(self.0.take())) };
            }
        }
    }

    #[test]
    fn test_handle() {
        let mut handle = NumberHandle::new(42);
        assert_eq!(handle.as_ref(), &42);
        assert_eq!(unsafe { *handle.as_ptr() }, 42);
        assert_eq!(unsafe { *handle.as_mut_ptr() }, 42);
        assert!(!handle.is_null());
        let dropped_value = handle.take();
        assert!(handle.is_null());
        assert_eq!(unsafe { *dropped_value }, 42);
        let other_handle: Option<Handle<bool>> = Handle::from_raw(std::ptr::null_mut());
        assert!(other_handle.is_none());
    }
}
