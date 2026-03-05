use crate::internal::hash::DefaultState;
use crate::{AnyRef, Arena};
use core::hash::BuildHasher;
use hashbrown::{HashMap, HashSet};

/// A secondary map that associates arena-allocated items with another value.
/// Items must be from the same arena.
pub struct SecondaryMap<T: ?Sized, V, S: BuildHasher = DefaultState> {
    map: HashMap<*const T, V, S>,
    rc: Arena<T>,
}
impl<T: ?Sized, V> SecondaryMap<T, V, DefaultState> {
    /// Creates a new map for items from the specified `Arena`.
    pub fn new(rc: Arena<T>) -> Self {
        Self {
            map: HashMap::with_hasher(DefaultState::new()),
            rc
        }
    }
}
impl<T: ?Sized, V, S: BuildHasher> SecondaryMap<T, V, S> {
    /// Returns `true` if the map contains a value for the specified key.
    #[inline]
    pub fn contains<K: AnyRef<T>>(&self, key: &K) -> bool {
        self.map.contains_key(&key.as_ptr())
    }

    /// Returns a reference to the value corresponding to the key, if one exists.
    #[inline]
    pub fn get<K: AnyRef<T>>(&self, key: &K) -> Option<&V> {
        self.map.get(&key.as_ptr())
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    #[inline]
    pub fn insert<K: AnyRef<T>>(&mut self, key: K, val: V) -> Option<V> {
        assert!(key.owned_by(&self.rc), "Arena does not own this key");
        self.map.insert(key.as_ptr(), val)
    }

    /// Removes a key from the map, returning the value if one was previously in the map.
    #[inline]
    pub fn remove<K: AnyRef<T>>(&mut self, key: &K) -> Option<V> {
        self.map.remove(&key.as_ptr())
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

/// A secondary set that references a subset of the arena
pub struct SecondarySet<T: ?Sized, S: BuildHasher = DefaultState> {
    set: HashSet<*const T, S>,
    rc: Arena<T>,
}
impl<T: ?Sized> SecondarySet<T, DefaultState> {
    /// Creates a new set for items from the specified `Arena`.
    pub fn new(rc: Arena<T>) -> Self {
        Self {
            set: HashSet::with_hasher(DefaultState::new()),
            rc
        }
    }
}
impl<T: ?Sized, S: BuildHasher> SecondarySet<T, S> {
    /// Returns `true` if the set contains the specified key.
    #[inline]
    pub fn contains<K: AnyRef<T>>(&self, key: &K) -> bool {
        self.set.contains(&key.as_ptr())
    }

    /// Inserts a key into the set.
    ///
    /// Returns whether the key was newly inserted.
    #[inline]
    pub fn insert<K: AnyRef<T>>(&mut self, key: K) -> bool {
        assert!(key.owned_by(&self.rc), "Arena does not own this key");
        self.set.insert(key.as_ptr())
    }

    /// Removes a key from the set.
    ///
    /// Returns whether the key was present in the set.
    #[inline]
    pub fn remove<K: AnyRef<T>>(&mut self, key: &K) -> bool {
        self.set.remove(&key.as_ptr())
    }

    /// Clears the map, removing all keys. Keeps the allocated memory for reuse.
    #[inline]
    pub fn clear(&mut self) {
        self.set.clear();
    }
}
