use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr::NonNull;
use crate::{Arena, Rel};
use crate::internal::ptr::non_null_deref;

/// Reference-counted pointer to a single item in an arena
pub struct Item<T: ?Sized> {
    ptr: NonNull<T>,
    rc: Arena<T>,
}
impl<T: ?Sized> Item<T> {
    pub(crate) fn new(ptr: NonNull<T>, rc: Arena<T>) -> Self {
        Self { ptr, rc }
    }
    
    pub(crate) fn arena(&self) -> &Arena<T> {
        &self.rc
    }
    
    pub fn rel(&self) -> Rel<T> {
        Rel::new(self.ptr, self.rc.downgrade())
    }
    
    pub(crate) fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn is(&self, other: &Self) -> bool {
        core::ptr::eq(self.as_ptr(), other.as_ptr())
    }

    #[inline]
    pub fn is_not(&self, other: &Self) -> bool {
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
impl<T: ?Sized> Clone for Item<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            rc: self.rc.clone(),
        }
    }
}
impl<T: ?Sized> Hash for Item<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}
impl<T: ?Sized> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Item<T> {}
impl<T: ?Sized> Deref for Item<T> {
    type Target = T;

    fn deref(&self) -> &T {
        non_null_deref(&self.ptr)
    }
}
impl<T: ?Sized + Display> Display for Item<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}
impl<T: ?Sized + Debug> Debug for Item<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}
