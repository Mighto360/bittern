use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use crate::rc::arena::{Arena, ArenaWeak};
use core::ptr::NonNull;
use crate::rc::item::Item;

/// Weakly reference counted pointer to a single item in an arena
pub struct Rel<T: ?Sized> {
    ptr: NonNull<T>,
    rc: ArenaWeak<T>,
}
impl<T: ?Sized> Rel<T> {
    pub(crate) fn new(ptr: NonNull<T>, rc: ArenaWeak<T>) -> Self {
        Self { ptr, rc }
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
    pub fn is_item(&self, other: &Item<T>) -> bool {
        core::ptr::eq(self.rc.as_ptr(), other.arena().as_ptr())
            && core::ptr::eq(self.as_ptr(), other.as_ptr())
    }

    #[inline]
    pub fn is_not_item(&self, other: &Item<T>) -> bool {
        !self.is_item(other)
    }

    fn arena(&self) -> Option<Arena<T>> {
       Arena::upgrade(&self.rc)
    }

    pub fn item(&self) -> Option<Item<T>> {
        match self.arena() {
            Some(rc) => Some(Item::new(self.ptr, rc)),
            None => None
        }
    }
}
impl<T: ?Sized> From<Item<T>> for Rel<T> {
    fn from(sym: Item<T>) -> Self {
        sym.rel()
    }
}
impl<T: ?Sized> Clone for Rel<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            rc: self.rc.clone(),
        }
    }
}
impl<T: ?Sized> Hash for Rel<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}
impl<T: ?Sized> PartialEq for Rel<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Rel<T> {}
impl<T: ?Sized + Debug> Debug for Rel<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rel({:x})", self.as_ptr().addr())
    }
}
