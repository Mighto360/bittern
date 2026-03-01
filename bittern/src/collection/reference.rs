use crate::collection::any_ref::AnyRef;
use crate::internal::ptr::{non_null, non_null_deref_copy};
use crate::{Arena, Strong, Weak};
use core::fmt::{Debug, Display};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr::NonNull;

/// Simple reference to a single item in an arena
pub struct Ref<'a, T: ?Sized> {
    r: &'a T,
    a: &'a Arena<T>,
}
impl<'a, T: ?Sized> Ref<'a, T> {
    pub(crate) fn new(ptr: NonNull<T>, arena: &'a Arena<T>) -> Ref<'a, T> {
        Self { r: non_null_deref_copy(ptr), a: arena }
    }

    pub fn arena(&self) -> &Arena<T> {
        &self.a
    }
    
    pub fn strong(&self) -> Strong<T> {
        Strong::new(non_null(self.r), self.a.clone())
    }

    pub fn weak(&self) -> Weak<T> {
        Weak::new(non_null(self.r), self.a.downgrade())
    }

    #[inline]
    pub fn is<O: AnyRef<T>>(&self, other: &O) -> bool {
        core::ptr::eq(self.as_ptr(), other.as_ptr())
    }

    #[inline]
    pub fn is_not<O: AnyRef<T>>(&self, other: &O) -> bool {
        !self.is(other)
    }
}
impl<T: ?Sized> AnyRef<T> for Ref<'_, T> {
    fn as_ptr(&self) -> *const T {
        self.r
    }

    fn owned_by(&self, arena: &Arena<T>) -> bool {
        arena.is(self.a)
    }
}
impl<T: ?Sized> Clone for Ref<'_, T> {
    fn clone(&self) -> Self {
        Self { r: self.r, a: self.a }
    }
}
impl<T: ?Sized> Copy for Ref<'_, T> {}
impl<T: ?Sized + Hash> Hash for Ref<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state)
    }
}
impl<T: ?Sized> PartialEq for Ref<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Ref<'_, T> {}
impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&'_ self) -> &'_ T {
        self.r
    }
}
impl<T: ?Sized + Display> Display for Ref<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.r.fmt(f)
    }
}
impl<T: ?Sized + Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.r.fmt(f)
    }
}
