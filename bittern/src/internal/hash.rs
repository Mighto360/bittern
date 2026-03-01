use core::hash::{BuildHasher, Hash, Hasher};
use ahash::RandomState;

pub(crate) type DefaultState = RandomState;

/// Uses the core::hash traits to hash a value
pub(crate) fn core_hash<K, S>(key: K, state: &S) -> u64
where K: Hash, S: BuildHasher
{
    let mut hasher = state.build_hasher();
    key.hash(&mut hasher);
    hasher.finish()
}
