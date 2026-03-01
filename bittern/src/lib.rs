#![no_std]
extern crate alloc;

mod identity;
mod rc;
mod internal;
mod config;

#[cfg(feature = "derive")]
pub use bittern_derive::*;

pub use config::ArenaConfig;
pub use rc::arena::Arena;
pub use rc::item::Item;
pub use rc::rel::Rel;
pub use rc::entry::Entry;
pub use rc::secondary::SecondaryMap;
pub use identity::Identity;

#[cfg(test)]
mod tests {
    use crate::{Arena, Item};

    #[test]
    fn test_arena_utilities() {
        let arena: Arena<str> = Arena::new();
        let s1 = arena.intern("hello");
        assert!(arena.owns(&s1));
        assert!(arena.contains("hello"));
        assert!(!arena.contains("world"));
    }

    #[test]
    fn test_str_identity_between_arenas() {
        let arena1: Arena<str> = Arena::new();
        let arena2: Arena<str> = Arena::new();
        let s1 = arena1.intern("hello");
        let s2 = arena2.intern("hello");
        assert!(arena1.owns(&s1));
        assert!(!arena1.owns(&s2));
        assert_eq!(&*s1, &*s2);
        assert!(s1.is_not(&s2));
    }

    #[test]
    fn test_str_identity_within_arena() {
        let arena: Arena<str> = Arena::new();
        let s1 = arena.intern("hello");
        let s2 = arena.intern("hello");
        let s3 = arena.intern("world");
        assert_eq!(&*s1, &*s2);
        assert!(s1.is(&s2));
        assert!(s1.is_not(&s3));
    }

    #[test]
    fn test_rc_safety() {
        let s1: Item<str>;
        {
            let arena: Arena<str> = Arena::new();
            {
                s1 = arena.intern("hello");
                assert_eq!(&*s1, "hello");
            }
            assert_eq!(&*s1, "hello");
        }
        assert_eq!(&*s1, "hello");
    }
}
