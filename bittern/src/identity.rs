use core::hash::{BuildHasher, Hash};
use crate::internal::hash::core_hash;

/// Minimum requirements for indexing items by a key type (or Self).
/// If more than one Identity impl exists for a type,
/// they must all produce the same hash for equivalent values.
pub trait Identity<K: ?Sized = Self> {
    type Index: ?Sized + Hash;

    /// Tests if values represent the same entity
    fn equivalent(&self, other: &K) -> bool;
    
    /// Provides a hashable value for each key.
    /// Values should be equivalent if and only if they have the same index
    fn index(key: &K) -> &Self::Index;
    
    /// Hashes a key for indexing
    fn hash<S: BuildHasher>(key: &K, state: &S) -> u64 {
        core_hash(Self::index(key), state)
    }
}

macro_rules! impl_identity {
    (<($($generic:tt)*)> $T:tt) => {
        impl<$($generic)*> Identity for $T where $T: Hash + Eq {
            type Index = Self;

            fn equivalent(&self, other: &Self) -> bool {
                self == other
            }

            fn index(key: &Self) -> &Self {
                key
            }
        }
    };
    ($($T:ty)+) => {
        $(
        impl Identity for $T {
            type Index = Self;

            fn equivalent(&self, other: &Self) -> bool {
                self == other
            }

            fn index(key: &Self) -> &Self {
                key
            }
        }
        )+
    };
}
impl_identity!(<(T)> [T]);
impl_identity!(<(T, const N: usize)> [T; N]);
impl_identity!(
    str char bool
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
