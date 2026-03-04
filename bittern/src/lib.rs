#![no_std]
extern crate alloc;

mod identity;
mod collection;
mod internal;
mod config;

#[cfg(feature = "derive")]
pub use bittern_derive::*;

pub use collection::arena::Arena;
pub use collection::reference::Ref;
pub use collection::secondary::{SecondaryMap, SecondarySet};
pub use collection::strong::Strong;
pub use collection::weak::Weak;
pub use collection::iter::ArenaIter;
pub use config::ArenaConfig;
pub use identity::Identity;

#[cfg(test)]
mod tests {
    use crate::{Arena, Strong};

    #[test]
    fn test_reference_comparison() {
        let arena: Arena<str> = Arena::new();
        let r1 = arena.intern("hello");
        let s1 = r1.strong();
        let w1 = r1.weak();

        let r2 = arena.intern("world");
        let s2 = r2.strong();
        let w2 = r2.weak();

        assert_eq!(arena.strong_count(), 3);
        assert_eq!(arena.weak_count(), 2);
        assert_eq!(arena.len(), 2);
        assert!(arena.allocation_size() >= 10); // exact allocation size can't be predicted

        // Multiple ways to create the same reference types
        assert!(r1.is(&s1.borrow()));
        assert!(w1.is(&s1.weak()));
        assert!(s1.is(&w1.strong().expect("failed to upgrade")));

        // Ref == Ref
        assert!(r1.is(&r1));
        assert!(r1.is_not(&r2));
        // Strong == Strong
        assert!(s1.is(&s1));
        assert!(s1.is_not(&s2));
        // Weak == Weak
        assert!(w1.is(&w1));
        assert!(w1.is_not(&w2));
        // Ref == Strong
        assert!(r1.is(&s1) && s1.is(&r1));
        assert!(r1.is_not(&s2) && s2.is_not(&r1));
        // Ref == Weak
        assert!(r1.is(&w1) && w1.is(&r1));
        assert!(r1.is_not(&w2) && w2.is_not(&r1));
        // Strong == Weak
        assert!(s1.is(&w1) && w1.is(&s1));
        assert!(s1.is_not(&w2) && w2.is_not(&s1));
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
        let s1: Strong<str>;
        {
            let arena: Arena<str> = Arena::new();
            {
                s1 = arena.intern("hello").strong();
                assert_eq!(&*s1, "hello");
            }
            assert_eq!(&*s1, "hello");
        }
        assert_eq!(&*s1, "hello");
    }
}
