use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use crate::collection::arena::{Arena, ArenaWeak};
use core::ptr::NonNull;
use crate::collection::any_ref::AnyRef;
use crate::collection::strong::Strong;

/// Weakly reference counted pointer to a single item in an arena
pub struct Weak<T: ?Sized> {
    ptr: NonNull<T>,
    rc: ArenaWeak<T>,
}
impl<T: ?Sized> Weak<T> {
    pub(crate) fn new(ptr: NonNull<T>, rc: ArenaWeak<T>) -> Self {
        Self { ptr, rc }
    }

    pub fn arena(&self) -> Option<Arena<T>> {
       Arena::upgrade(&self.rc)
    }

    pub fn strong(&self) -> Option<Strong<T>> {
        match self.arena() {
            Some(rc) => Some(Strong::new(self.ptr, rc)),
            None => None
        }
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
impl<T: ?Sized> AnyRef<T> for Weak<T> {
    fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    fn owned_by(&self, arena: &Arena<T>) -> bool {
        arena.is_inner(self.rc.as_ptr())
    }
}
impl<T: ?Sized> From<Strong<T>> for Weak<T> {
    fn from(sym: Strong<T>) -> Self {
        sym.weak()
    }
}
impl<T: ?Sized> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            rc: self.rc.clone(),
        }
    }
}
impl<T: ?Sized> Hash for Weak<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}
impl<T: ?Sized> PartialEq for Weak<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Weak<T> {}
impl<T: ?Sized + Debug> Debug for Weak<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Weak({})", core::any::type_name::<T>())
    }
}
