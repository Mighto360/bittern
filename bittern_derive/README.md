# bittern_derive

[bittern]: https://github.com/mighto360/bittern/blob/main/bittern

Procedural macros for deriving [bittern] traits.

```rust
use bittern::Identity;

// Derive Identity for any type that is Hash + Eq
#[derive(Identity, Hash, Eq, PartialEq)]
struct SimpleIdentity {
    key: u64
}

// Derive Identity using by delegating to a single "key" field that implements Identity.
// The other fields don't have to be Hash + Eq
#[derive(Identity)]
struct NestedIdentity {
    #[identity]
    key: u64,
    other_field: f32,
}
```
