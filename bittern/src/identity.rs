use core::hash::{BuildHasher, Hash};
use crate::internal::hash::core_hash;

/// Minimum requirements for indexing items by a key type (or Self).
/// If more than one Identity impl exists for a type,
/// they must all produce the same hash for equivalent values.
pub trait Identity<K: ?Sized = Self> {
    /// Tests if values represent the same entity
    fn equivalent(&self, other: &K) -> bool;
    
    /// Provides a hashable value for each key.
    /// Values should be equivalent if and only if they have the same index
    fn index(key: &K) -> impl Hash;
    
    /// Hashes a key for indexing
    fn hash<S: BuildHasher>(key: &K, state: &S) -> u64 {
        core_hash(Self::index(key), state)
    }
}

impl<T> Identity for T where T: ?Sized + Hash + Eq {
    fn equivalent(&self, other: &Self) -> bool {
        self == other
    }

    fn index(key: &Self) -> impl Hash {
        key
    }
}
