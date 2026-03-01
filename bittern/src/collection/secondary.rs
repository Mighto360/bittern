use crate::collection::any_ref::AnyRef;
use crate::internal::hash::DefaultState;
use crate::Arena;
use core::hash::BuildHasher;
use hashbrown::HashMap;

/// A secondary map that associates arena-allocated items with another value.
/// Items must be from the same arena.
pub struct SecondaryMap<T: ?Sized, V, S: BuildHasher = DefaultState> {
    map: HashMap<*const T, V, S>,
    rc: Arena<T>,
}
impl<T: ?Sized, V> SecondaryMap<T, V, DefaultState> {
    pub fn new(rc: Arena<T>) -> Self {
        Self {
            map: HashMap::with_hasher(DefaultState::new()),
            rc
        }
    }
}
impl<T: ?Sized, V, S: BuildHasher> SecondaryMap<T, V, S> {
    #[inline]
    pub fn contains<K: AnyRef<T>>(&self, key: &K) -> bool {
        self.map.contains_key(&key.as_ptr())
    }
    
    #[inline]
    pub fn get<K: AnyRef<T>>(&self, key: &K) -> Option<&V> {
        self.map.get(&key.as_ptr())
    }

    #[inline]
    pub fn insert<K: AnyRef<T>>(&mut self, key: K, val: V) -> Option<V> {
        assert!(key.owned_by(&self.rc), "Arena does not own this key");
        self.map.insert(key.as_ptr(), val)
    }

    #[inline]
    pub fn remove<K: AnyRef<T>>(&mut self, key: &K) -> Option<V> {
        self.map.remove(&key.as_ptr())
    }
    
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }
}
