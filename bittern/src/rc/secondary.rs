use crate::internal::hash::DefaultState;
use crate::Item;
use core::hash::BuildHasher;
use hashbrown::HashMap;

/// A secondary map that associates Items with another value.
/// Items do not have to be from the same arena.
pub struct SecondaryMap<T: ?Sized, V, S: BuildHasher = DefaultState> {
    map: HashMap<Item<T>, V, S>,
}
impl<T: ?Sized, V> SecondaryMap<T, V, DefaultState> {
    pub fn new() -> Self {
        Self {
            map: HashMap::with_hasher(DefaultState::new())
        }
    }
}
impl<T: ?Sized, V, S: BuildHasher> SecondaryMap<T, V, S> {
    #[inline]
    pub fn contains(&self, key: &Item<T>) -> bool {
        self.map.contains_key(key)
    }
    
    #[inline]
    pub fn get(&self, key: &Item<T>) -> Option<&V> {
        self.map.get(key)
    }

    #[inline]
    pub fn insert(&mut self, key: Item<T>, val: V) -> Option<V> {
        self.map.insert(key, val)
    }

    #[inline]
    pub fn remove(&mut self, key: &Item<T>) -> Option<V> {
        self.map.remove(key)
    }
    
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }
}
