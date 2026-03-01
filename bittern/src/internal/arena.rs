use crate::identity::Identity;
use crate::internal::index::HashIndex;
use crate::internal::ptr::{non_null, non_null_drop};
use crate::ArenaConfig;
use bumpalo::Bump;
use core::cell::RefCell;
use core::mem::size_of;
use core::ptr::NonNull;

pub(crate) struct ArenaInner<T: ?Sized> {
    arena: Bump,
    index: RefCell<HashIndex<T>>,
    config: ArenaConfig,
}
impl<T: ?Sized> ArenaInner<T> {
    pub(crate) fn new(config: ArenaConfig) -> Self {
        Self {
            arena: Bump::new(),
            index: RefCell::new(HashIndex::new()),
            config,
        }
    }
    
    pub(crate) fn config(&self) -> &ArenaConfig {
        &self.config
    }

    pub(crate) fn for_each(&self, f: impl FnMut(&T)) {
        let index = self.index.borrow();
        index.iter_ref().for_each(f)
    }

    pub(crate) fn allocation_size(&self) -> usize {
        let base_size = size_of::<ArenaInner<T>>();
        let index_size = self.index.borrow().allocation_size();
        let arena_size = self.arena.allocated_bytes_including_metadata();
        base_size + index_size + arena_size
    }

    pub(crate) fn len(&self) -> usize {
        self.index.borrow().len()
    }

    pub(crate) fn contains<K>(&self, key: &K) -> bool
    where K: ?Sized, T: Identity<K>
    {
        self.index.borrow().contains(key)
    }

    pub(crate) fn get_ptr<K>(&self, key: &K) -> Option<NonNull<T>>
    where K: ?Sized, T: Identity<K>
    {
        self.index.borrow().get_ptr(key)
    }
}
impl<T: ?Sized> Drop for ArenaInner<T> {
    fn drop(&mut self) {
        if self.config.drop_items {
            for ptr in self.index.borrow().iter() {
                non_null_drop(ptr);
            }
        }
    }
}
// intern_owned
impl<T: Identity> ArenaInner<T> where Self: AllocOwned<T> {
    pub(crate) fn intern_owned(&self, val: T) -> NonNull<T> {
        match self.get_ptr(&val) {
            Some(existing_ptr) => existing_ptr,
            None => {
                let new: &mut T = self.alloc(val);
                self.index.borrow_mut().insert_unique(new);
                non_null(new)
            }
        }
    }
}
// intern
impl<T: ?Sized + Identity> ArenaInner<T> where Self: AllocCopy<T> {
    pub(crate) fn intern(&self, val: &T) -> NonNull<T> {
        match self.get_ptr(val) {
            Some(existing_ptr) => existing_ptr,
            None => {
                let new: &mut T = self.alloc(val);
                self.index.borrow_mut().insert_unique(new);
                non_null(new)
            }
        }
    }
}
// intern_cloned
impl<T: ?Sized + Identity> ArenaInner<T> where Self: AllocClone<T> {
    pub(crate) fn intern_cloned(&self, val: &T) -> NonNull<T> {
        match self.get_ptr(val) {
            Some(existing_ptr) => existing_ptr,
            None => {
                let new: &mut T = self.alloc_cloned(val);
                self.index.borrow_mut().insert_unique(new);
                non_null(new)
            }
        }
    }
}

/// Allocates by moving owned value to the heap
pub(crate) trait AllocOwned<T: ?Sized> {
    fn alloc(&self, val: T) -> &mut T;
}
impl<T> AllocOwned<T> for ArenaInner<T> {
    fn alloc(&self, val: T) -> &mut T {
        self.arena.alloc(val)
    }
}

/// Allocates by copying value to the heap
pub(crate) trait AllocCopy<T: ?Sized> {
    fn alloc(&self, val: &T) -> &mut T;
}
impl AllocCopy<str> for ArenaInner<str> {
    fn alloc(&self, val: &str) -> &mut str {
        self.arena.alloc_str(val)
    }
}
impl<T: Copy> AllocCopy<[T]> for ArenaInner<[T]> {
    fn alloc(&self, val: &[T]) -> &mut [T] {
        self.arena.alloc_slice_copy(val)
    }
}

/// Allocates by cloning value to the heap
pub(crate) trait AllocClone<T: ?Sized> {
    fn alloc_cloned(&self, val: &T) -> &mut T;
}
impl<T: Clone> AllocClone<T> for ArenaInner<T> {
    fn alloc_cloned(&self, val: &T) -> &mut T {
        self.arena.alloc(val.clone())
    }
}
impl<T: Clone> AllocClone<[T]> for ArenaInner<[T]> {
    fn alloc_cloned(&self, val: &[T]) -> &mut [T] {
        self.arena.alloc_slice_clone(val)
    }
}
