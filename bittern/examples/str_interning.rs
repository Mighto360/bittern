// Demonstrates how to intern strings, without managing lifetimes or extra indirection from String

use bittern::{Arena, Item};

fn main() {
    // Create an arena
    let arena: Arena<str> = Arena::new();

    // Allocate a new str in the arena.
    // The lifetime of the slice doesn't matter, since it is copied into heap memory
    let s1: Item<str> = arena.intern("hello world");

    // This str is already interned so it will return the same item
    let s2: Item<str> = arena.intern("hello world");
    assert!(s2.is(&s1));

    // This str is new, it will return a different item
    let s3: Item<str> = arena.intern("👋🌎");
    assert!(s3.is_not(&s1));

    // Comparing items by identity is much faster than a string equality comparison
    assert!(s1.is(&s2));    // ~0.3 ns
    assert_eq!(&*s1, &*s2); // ~4.0 ns (more than 10x slower, even for short strings!)
}
