use crate::collection::any_ref::AnyRef;
use crate::collection::entry::{Entry, EntryOccupied, EntryVacant};
use crate::collection::reference::Ref;
use crate::identity::Identity;
use crate::internal::arena::ArenaInner;
use crate::ArenaConfig;
use alloc::rc::{Rc, Weak};

pub(crate) type ArenaWeak<T> = Weak<ArenaInner<T>>;

/// Reference-counted pointer to an arena
pub struct Arena<T: ?Sized> {
    rc: Rc<ArenaInner<T>>
}
impl<T: ?Sized> Arena<T> {
    pub fn new() -> Self {
        Self::with_config(ArenaConfig::default())
    }
    
    pub fn with_config(config: ArenaConfig) -> Self {
        Self { rc: Rc::new(ArenaInner::new(config)) }
    }

    #[inline]
    pub fn config(&self) -> &ArenaConfig {
        self.rc.config()
    }

    pub(crate) fn upgrade(weak: &ArenaWeak<T>) -> Option<Self> {
        match weak.upgrade() {
            Some(rc) => Some(Self { rc }),
            None => None
        }
    }

    pub(crate) fn downgrade(&self) -> ArenaWeak<T> {
        Rc::downgrade(&self.rc)
    }

    pub(crate) fn as_ptr(&self) -> *const ArenaInner<T> {
        self.rc.as_ref()
    }

    pub(crate) fn is_inner(&self, other: *const ArenaInner<T>) -> bool {
        core::ptr::addr_eq(self.as_ptr(), other)
    }

    #[inline]
    pub fn is(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }

    #[inline]
    pub fn is_not(&self, other: &Self) -> bool {
        !self.is(other)
    }

    #[inline]
    pub fn owns<K: AnyRef<T>>(&self, key: &K) -> bool {
        key.owned_by(self)
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.rc)
    }
    
    #[inline]
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.rc)
    }

    #[inline]
    pub fn allocation_size(&self) -> usize {
        self.rc.allocation_size()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.rc.len()
    }
    
    #[inline]
    pub fn for_each(&self, f: impl FnMut(&T)) {
        self.rc.for_each(f)
    }
    
    pub fn get<K>(&'_ self, key: &K) -> Option<Ref<'_, T>>
    where K: ?Sized, T: Identity<K>
    {
        match self.rc.get_ptr(key) {
            Some(ptr) => Some(Ref::new(ptr, self)),
            None => None,
        }
    }

    pub fn entry<K>(&'_ self, key: &K) -> Entry<'_, T>
    where K: ?Sized, T: Identity<K>
    {
        match self.get(key) {
            Some(item) => Entry::Occupied(EntryOccupied { item }),
            None => Entry::Vacant(EntryVacant { arena: self }),
        }
    }
    
    pub fn contains<K>(&self, key: &K) -> bool
    where K: ?Sized, T: Identity<K>
    {
        self.rc.contains(key)
    }
}
impl<T: ?Sized> Clone for Arena<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}

// intern
impl Arena<str> {
    pub fn intern(&'_ self, val: &str) -> Ref<'_, str>{
        Ref::new(self.rc.intern(val), self)
    }
}
impl<T> Arena<[T]> where T: Copy, [T]: Identity
{
    pub fn intern(&'_ self, val: &[T]) -> Ref<'_, [T]> {
        Ref::new(self.rc.intern(val), self)
    }
}

// intern_owned
impl<T> Arena<T> where T: Identity
{
    pub fn intern_owned(&'_ self, val: T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_owned(val), self)
    }
}

// intern_cloned
impl<T> Arena<T> where T: Clone + Identity
{
    pub fn intern_cloned(&'_ self, val: &T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_cloned(val), self)
    }
}
impl<T> Arena<[T]> where T: Clone, [T]: Identity
{
    pub fn intern_cloned(&'_ self, val: &[T]) -> Ref<'_, [T]> {
        Ref::new(self.rc.intern_cloned(val), self)
    }
}
