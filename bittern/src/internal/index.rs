use crate::internal::hash::DefaultState;
use crate::identity::Identity;
use crate::internal::ptr::{non_null, non_null_deref};
use core::hash::BuildHasher;
use core::ptr::NonNull;
use hashbrown::HashTable;
use crate::internal::iter::IndexIter;

/// Hash table that doesn't store values (only their pointer)
pub(crate) struct HashIndex<T: ?Sized, S: BuildHasher = DefaultState> {
    state: S,
    table: HashTable<NonNull<T>>,
}
impl<T: ?Sized> HashIndex<T, DefaultState> {
    pub(crate) fn new() -> Self {
        Self {
            state: DefaultState::new(),
            table: HashTable::new(),
        }
    }
}
impl<T: ?Sized, S: BuildHasher> HashIndex<T, S> {
    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn allocation_size(&self) -> usize {
        self.table.allocation_size()
    }

    pub(crate) fn iter(&'_ self) -> IndexIter<'_, T> {
        IndexIter::new(self.table.iter())
    }
}
impl<T: ?Sized, S: BuildHasher> HashIndex<T, S> {
    pub(crate) fn get_ptr<K>(&self, key: &K) -> Option<NonNull<T>>
    where K: ?Sized, T: Identity<K>
    {
        let hash = T::hash(key, &self.state);
        let eq = |other: &NonNull<T>| non_null_deref(other).equivalent(key);
        self.table.find(hash, eq).copied()
    }

    pub(crate) fn contains<K>(&self, key: &K) -> bool
    where K: ?Sized, T: Identity<K>
    {
        self.get_ptr(key).is_some()
    }
}
impl<T: ?Sized + Identity, S: BuildHasher> HashIndex<T, S> {
    pub(crate) fn insert_unique(&mut self, val: &mut T) {
        let hash = T::hash(val, &self.state);
        let rehash = |v: &NonNull<T>| T::hash(non_null_deref(v), &self.state);
        self.table.insert_unique(hash, non_null(val), rehash);
    }
}
