use crate::collection::any_ref::AnyRef;
use crate::collection::reference::Ref;
use crate::identity::Identity;
use crate::internal::arena::ArenaInner;
use crate::ArenaConfig;
use alloc::rc::{Rc, Weak};
use crate::collection::iter::ArenaIter;

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

    pub(crate) fn is_inner(&self, other: *const ArenaInner<T>) -> bool {
        core::ptr::addr_eq(self.as_ptr(), other)
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
    pub fn owns<K: AnyRef<T>>(&self, key: &K) -> bool {
        key.owned_by(self)
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

    pub fn iter(&'_ self) -> ArenaIter<'_, T> {
        ArenaIter::new(self.rc.iter(), self)
    }
    
    pub fn get<K>(&'_ self, key: &K) -> Option<Ref<'_, T>>
    where K: ?Sized, T: Identity<K>
    {
        match self.rc.get_ptr(key) {
            Some(ptr) => Some(Ref::new(ptr, self)),
            None => None,
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
    pub fn intern(&'_ self, val: &str) -> Ref<'_, str>{
        Ref::new(self.rc.intern(val), self)
    }
}
impl<T> Arena<[T]> where T: Copy, [T]: Identity
{
    pub fn intern(&'_ self, val: &[T]) -> Ref<'_, [T]> {
        Ref::new(self.rc.intern(val), self)
    }
}

// intern_owned
impl<T> Arena<T> where T: Identity
{
    pub fn intern_owned(&'_ self, val: T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_owned(val), self)
    }
}

// intern_cloned
impl<T> Arena<T> where T: Clone + Identity
{
    pub fn intern_cloned(&'_ self, val: &T) -> Ref<'_, T> {
        Ref::new(self.rc.intern_cloned(val), self)
    }
}
impl<T> Arena<[T]> where T: Clone, [T]: Identity
{
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
