use crate::identity::Identity;
use crate::internal::arena::ArenaInner;
use crate::rc::item::Item;
use alloc::rc::{Rc, Weak};
use core::cell::Ref;
use core::ptr::NonNull;
use crate::ArenaConfig;
use crate::internal::index::HashIndex;
use crate::rc::entry::{Entry, EntryOccupied, EntryVacant};

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
    
    fn symbol_ref(&self, ptr: NonNull<T>) -> Item<T> {
        Item::new(ptr, self.clone())
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
    pub fn owns(&self, key: &Item<T>) -> bool {
        self.is(key.arena())
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

    pub fn get<K>(&self, key: &K) -> Option<Item<T>>
    where K: ?Sized, T: Identity<K>
    {
        match self.rc.get(key) {
            Some(ptr) => Some(self.symbol_ref(ptr)),
            None => None
        }
    }

    pub fn entry<K>(&self, key: &K) -> Entry<T>
    where K: ?Sized, T: Identity<K>
    {
        match self.get(key) {
            Some(item) => Entry::Occupied(EntryOccupied { item }),
            None => Entry::Vacant(EntryVacant { arena: self.clone() }),
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
    pub fn intern(&self, val: &str) -> Item<str> {
        self.symbol_ref(self.rc.intern(val))
    }
}
impl<T> Arena<[T]> where T: Copy, [T]: Identity
{
    pub fn intern(&self, val: &[T]) -> Item<[T]> {
        self.symbol_ref(self.rc.intern(val))
    }
}

// intern_owned
impl<T> Arena<T> where T: Identity
{
    pub fn intern_owned(&self, val: T) -> Item<T> {
        self.symbol_ref(self.rc.intern_owned(val))
    }
}

// intern_cloned
impl<T> Arena<T> where T: Clone + Identity
{
    pub fn intern_cloned(&self, val: &T) -> Item<T> {
        self.symbol_ref(self.rc.intern_cloned(val))
    }
}
impl<T> Arena<[T]> where T: Clone, [T]: Identity
{
    pub fn intern_cloned(&self, val: &[T]) -> Item<[T]> {
        self.symbol_ref(self.rc.intern_cloned(val))
    }
}
