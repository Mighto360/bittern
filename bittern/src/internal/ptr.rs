use core::ptr::NonNull;

/// Polyfill for NonNull::from_ref
/// # Safety
/// A reference cannot be null
pub(crate) fn non_null<T: ?Sized>(r: &T) -> NonNull<T> {
    unsafe { NonNull::new_unchecked(r as *const T as *mut T) }
}

/// Dereferences a referenced NonNull pointer
/// # Safety
/// Caller must guarantee that the pointer will be a safe reference
pub(crate) fn non_null_deref<T: ?Sized>(ptr: &NonNull<T>) -> &T {
    unsafe { ptr.as_ref() }
}


/// Dereferences an owned NonNull pointer
/// # Safety
/// Caller must guarantee that the pointer will be a safe reference, and the lifetime is correct
pub(crate) fn non_null_deref_copy<'a, T: ?Sized>(ptr: NonNull<T>) -> &'a T {
    unsafe { ptr.as_ref() }
}

/// Drops the data of NonNull pointer in place
/// # Safety
/// Caller must guarantee that the pointer can be safely dropped, and will not be accessed again
pub(crate) fn non_null_drop<T: ?Sized>(ptr: &NonNull<T>) {
    unsafe { core::ptr::drop_in_place(ptr.as_ptr()) }
}
