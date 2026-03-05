use crate::collection::reference::Ref;
use crate::identity::Identity;
use crate::internal::arena::ArenaInner;
use crate::{ArenaConfig, AnyRef};
use alloc::rc::{Rc, Weak};
use crate::collection::iter::ArenaIter;

pub(crate) type ArenaWeak<T> = Weak<ArenaInner<T>>;

/// Reference-counted pointer to an arena
pub struct Arena<T: ?Sized> {
    rc: Rc<ArenaInner<T>>
}
impl<T: ?Sized> Arena<T> {
    /// Creates an empty `Arena` with the default config.
    pub fn new() -> Self {
        Self::with_config(ArenaConfig::default())
    }

    /// Creates an empty `Arena` with a custom config.
    pub fn with_config(config: ArenaConfig) -> Self {
        Self { rc: Rc::new(ArenaInner::new(config)) }
    }

    /// Returns the configuration of an arena.
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
        core::ptr::eq(self.as_ptr(), other)
    }

    /// Returns `true` if two references point to the same arena.
    #[inline]
    pub fn is(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }

    /// Returns `true` if two references point to different arenas.
    /// Equivalent to `!self.is(other)`.
    #[inline]
    pub fn is_not(&self, other: &Self) -> bool {
        !self.is(other)
    }

    /// Returns `true` if the referenced item is allocated in this arena.
    #[inline]
    pub fn owns<R: AnyRef<T>>(&self, item: &R) -> bool {
        item.owned_by(self)
    }

    /// Returns the number of strong pointers to this arena.
    ///
    /// This includes `Arena` and `Strong` values that reference this allocation.
    #[inline]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.rc)
    }

    /// Returns the number of weak pointers to this arena.
    ///
    /// This includes `Weak` values that reference this allocation.
    #[inline]
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.rc)
    }

    /// Returns the total heap memory allocated by this arena, in bytes.
    ///
    /// This includes all allocated items, index, configuration, and other metadata.
    #[inline]
    pub fn allocation_size(&self) -> usize {
        self.rc.allocation_size()
    }

    /// Returns the number of items in this arena.
    #[inline]
    pub fn len(&self) -> usize {
        self.rc.len()
    }

    /// Returns an iterator over all items in this arena, in arbitrary order.
    pub fn iter(&'_ self) -> ArenaIter<'_, T> {
        ArenaIter::new(self.rc.iter(), self)
    }

    /// Returns a reference to an item identified by the specified key.
    ///
    /// The key may be any type where the item implements `Identity<K>`.
    pub fn get<K>(&'_ self, key: &K) -> Option<Ref<'_, T>>
    where K: ?Sized, T: Identity<K>
    {
        match self.rc.get_ptr(key) {
            Some(ptr) => Some(Ref::new(ptr, self)),
            None => None,
        }
    }

    /// Returns `true` if the arena contains an item identified by the specified key.
    ///
    /// The key may be any type where the item implements `Identity<K>`.
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
impl<T: ?Sized> PartialEq for Arena<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}
impl<T: ?Sized> Eq for Arena<T> {}

// intern
impl Arena<str> {
    /// Interns a value in the arena by copying it to the heap.
    ///
    /// If an identical value is found in the arena (see [`Identity`]), the existing value is returned.
    ///
    pub fn intern(&'_ self, val: &str) -> Ref<'_, str>{
        Ref::new(self.rc.intern(val), self)
    }
}
impl<T> Arena<[T]> where T: Copy, [T]: Identity
{
    /// Interns a value in the arena by copying it to the heap.
    ///
    pub fn intern(&'_ self, val: &[T]) -> Ref<'_, [T]> {
        Ref::new(self.rc.intern(val), self)
    }
}

// intern_owned
impl<T> Arena<T> where T: Identity
{
    /// Interns a value in the arena by moving it to the heap.
    ///
    pub fn intern_owned(&'_ self, val: T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_owned(val), self)
    }
}

// intern_cloned
impl<T> Arena<T> where T: Clone + Identity
{
    /// Interns a value in the arena by cloning it to the heap.
    ///
    pub fn intern_cloned(&'_ self, val: &T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_cloned(val), self)
    }
}
impl<T> Arena<[T]> where T: Clone, [T]: Identity
{
    /// Interns a value in the arena by cloning it to the heap.
    ///
    pub fn intern_cloned(&'_ self, val: &[T]) -> Ref<'_, [T]> {
        Ref::new(self.rc.intern_cloned(val), self)
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use crate::Arena;

    #[test]
    fn test_intern() {
        macro_rules! test_intern {
            (intern $T:ty = $val:expr) => {
                let arena: Arena<$T> = Arena::new();
                let ref1 = arena.intern($val);
                let ref2 = arena.intern($val);
                assert!(ref1.is(&ref2));
                assert_eq!(&*ref1, $val);
                assert_eq!(&*ref2, $val);
            };
            (intern_owned $T:ty = $val:expr) => {
                let arena: Arena<$T> = Arena::new();
                let ref1 = arena.intern_owned($val);
                let ref2 = arena.intern_owned($val);
                assert!(ref1.is(&ref2));
                assert_eq!(*ref1, $val);
                assert_eq!(*ref2, $val);
            };
            (intern_cloned $T:ty = $val:expr) => {
                let arena: Arena<$T> = Arena::new();
                let ref1 = arena.intern_cloned(&$val);
                let ref2 = arena.intern_cloned(&$val);
                assert!(ref1.is(&ref2));
                assert_eq!(*ref1, $val);
                assert_eq!(*ref2, $val);
            };
        }

        let test_i32: i32 = 123;
        test_intern!(intern_owned i32 = test_i32);
        test_intern!(intern_cloned i32 = test_i32);

        let test_i32_array: [i32; 3] = [1, 2, 3];
        test_intern!(intern_owned [i32; 3] = test_i32_array);
        test_intern!(intern_cloned [i32; 3] = test_i32_array);
        test_intern!(intern [i32] = &test_i32_array);
        test_intern!(intern_cloned [i32] = test_i32_array);

        let test_str: &str = "hello";
        test_intern!(intern str = test_str);
    }

    #[test]
    fn test_contains() {
        let arena: Arena<str> = Arena::new();
        arena.intern("hello");
        assert!(arena.contains("hello"));
        assert!(!arena.contains("world"));
    }

    #[test]
    fn test_get() {
        let arena: Arena<str> = Arena::new();
        let s1 = arena.intern("hello");
        let s2 = arena.get("hello").expect("failed to find item");
        assert!(s2.is(&s1));
        let s3 = arena.get("world");
        assert!(s3.is_none());
    }

    #[test]
    fn test_iter() {
        let arena: Arena<str> = Arena::new();
        let mut test_strings = ["hello", "world", "this", "is", "a", "test"];
        for s in test_strings {
            arena.intern(s);
        }
        let mut found_strings = arena.iter().collect::<Vec<_>>();
        assert_eq!(found_strings.len(), test_strings.len());
        // Compare strings
        found_strings.sort_by(|a, b| a.cmp(b));
        test_strings.sort();
        for i in 0..test_strings.len() {
            assert_eq!(test_strings[i], &*found_strings[i]);
        }
    }
}
