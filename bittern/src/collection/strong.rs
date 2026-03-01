use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr::NonNull;
use crate::{Arena, Ref, Weak};
use crate::collection::any_ref::AnyRef;
use crate::internal::ptr::non_null_deref;

/// Reference-counted pointer to a single item in an arena
pub struct Strong<T: ?Sized> {
    ptr: NonNull<T>,
    rc: Arena<T>,
}
impl<T: ?Sized> Strong<T> {
    pub(crate) fn new(ptr: NonNull<T>, rc: Arena<T>) -> Self {
        Self { ptr, rc }
    }
    
    pub fn arena(&self) -> &Arena<T> {
        &self.rc
    }
    
    pub fn weak(&self) -> Weak<T> {
        Weak::new(self.ptr, self.rc.downgrade())
    }

    pub fn borrow(&'_ self) -> Ref<'_, T> {
        Ref::new(self.ptr, &self.rc)
    }

    #[inline]
    pub fn is<O: AnyRef<T>>(&self, other: &O) -> bool {
        core::ptr::eq(self.as_ptr(), other.as_ptr())
    }

    #[inline]
    pub fn is_not<O: AnyRef<T>>(&self, other: &O) -> bool {
        !self.is(other)
    }
    
    #[inline]
    pub fn arena_strong_count(&self) -> usize {
        self.rc.strong_count()
    }

    #[inline]
    pub fn arena_weak_count(&self) -> usize {
        self.rc.weak_count()
    }
}
impl<T: ?Sized> AnyRef<T> for Strong<T> {
    fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    fn owned_by(&self, arena: &Arena<T>) -> bool {
        arena.is(&self.rc)
    }
}
impl<T: ?Sized> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            rc: self.rc.clone(),
        }
    }
}
impl<T: ?Sized> Hash for Strong<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}
impl<T: ?Sized> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Strong<T> {}
impl<T: ?Sized> Deref for Strong<T> {
    type Target = T;

    fn deref(&'_ self) -> &'_ T {
        non_null_deref(&self.ptr)
    }
}
impl<T: ?Sized + Display> Display for Strong<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}
impl<T: ?Sized + Debug> Debug for Strong<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}
