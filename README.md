# bittern

A reference-counted arena for interning and deduplicating data.

Bittern provides `Arena<T>`, a collection type with roughly three duties:

- Bump allocation: a blazing fast arena for any type, including dynamically sized slices.
- Indexing: items can be accessed and interned by their hash or unique key.
- Reference counting: the arena will live as long as any references to it or its data.

Bump allocators are appropriate for use cases where items will be allocated gradually, but dropped as a group.   
That makes bittern ideal for building large graphs, symbol tables, and other deduplicated collections.

## Compatibility

This crate fully supports `#![no_std]` environments. It depends only on `core` and `alloc`.

## Examples

### 1) String interning / Symbol table

When interning dynamically sized slices or strings,
bittern can store values together in a chunk of allocated memory.
- Fewer allocations and better locality compared to many individual `String` or `Vec`.
- Slices are interned into a pointer, which greatly improves the performance of equality checks.

[examples/str_interning.rs](bittern/examples/str_interning.rs)
```rust
// Demonstrates how to intern strings, without wrangling lifetimes or individual String allocations

use bittern::{Arena, Ref};

fn main() {
    // Create an arena
    let arena: Arena<str> = Arena::new();

    // Allocate a new str in the arena.
    // The lifetime of the slice doesn't matter, since it is copied into heap memory
    let s1: Ref<str> = arena.intern("hello world");

    // This str is already interned so it will return the same item
    let s2: Ref<str> = arena.intern("hello world");
    assert!(s2.is(&s1));

    // This str is new, it will return a different item
    let s3: Ref<str> = arena.intern("👋🌎");
    assert!(s3.is_not(&s1));

    // Comparing items by identity is much faster than a string equality comparison
    assert!(s1.is(&s2));    // ~0.3 ns
    assert_eq!(&*s1, &*s2); // ~4.0 ns (more than 10x slower, even for short strings!)
}
```

### 2) Abstract syntax tree

This crate is well suited for building graphs and trees with many identical nodes.  
The following example demonstrates a math interpreter that merges equivalent subexpressions.

[examples/parsing.rs](bittern/examples/parsing.rs)
```rust
// Demonstrates a simple expression interpreter using an arena-allocated syntax tree.
// The language uses Lisp-like prefix notation with optional parentheses

use bittern::{Arena, Strong, Weak, SecondaryMap};
use core::hash::Hash;

fn main() {
    // Evaluate the Pythagorean theorem (sqrt(3000^2 + 6000^2) = 6708)
    let input = r#"
    do
    let a 3000
    let b 6000
    let c sqrt (+ (pow a 2) (pow b 2))
    (c)
    "#;
    
    let mut parser = Parser::new();
    let expr = parser.parse(input).strong();
    assert_eq!(parser.expr_table.len(), 14);
    
    let result = Eval::new(&parser).eval(expr);
    assert_eq!(result, Some(6708));
}

type Int = i64;
type Name = str;

// An expression tree or subtree.
// Strong<Name> is a strong ref, so the Name arena will live until the Expr is dropped.
// Weak<Expr> is a weak ref, so expressions may reference others within the same arena.
#[derive(Hash, PartialEq, Eq, Debug)]
enum Expr {
    Empty,
    Int(Int),
    Name(Strong<Name>),
    Block(Vec<Weak<Expr>>),
    Let(Weak<Expr>, Weak<Expr>),
    Add(Weak<Expr>, Weak<Expr>),
    Sub(Weak<Expr>, Weak<Expr>),
    Mul(Weak<Expr>, Weak<Expr>),
    Div(Weak<Expr>, Weak<Expr>),
    Pow(Weak<Expr>, Weak<Expr>),
    Sqrt(Weak<Expr>),
}

// Parses input into an AST.
// Identical expressions will be interned into a single node.
struct Parser<'src> {
    input: &'src str,
    name_table: Arena<Name>,
    expr_table: Arena<Expr>,
}
impl<'src> Parser<'src> {
    // ... full impl in examples/parsing.rs
}

// Evaluates the AST.
// SecondaryMap associates a Strong<Name> with a value
struct Eval {
    var_table: SecondaryMap<Name, Option<Int>>,
}
impl Eval {
    // ... full impl in examples/parsing.rs
}
```


### 3) Primary keys

Sometimes data should be deduplicated by a single field, rather than the entire struct.  
This demonstrates a `User` struct identified by its `id` field.

[examples/primary_key.rs](bittern/examples/primary_key.rs)
```rust
// Demonstrates an arena of Users, deduplicated by their id field.
// Feature "derive" must be enabled

use core::cell::RefCell;
use bittern::{Identity, Arena, Strong, Ref};

fn main() {
    let users = Users::new();

    let u1: Ref<User> = users.insert_or_update(0, "John Doe");
    let u2: Ref<User> = users.insert_or_update(1, "Jane Doe");
    assert!(u1.is_not(&u2));

    let u3: Ref<User> = users.insert_or_update(0, "JOHN");
    assert!(u1.is(&u3));
    let name = u3.name.borrow();
    assert_eq!(&**name, "JOHN"); // This is the previously interned User, but the name was updated
}

#[derive(Identity)]
struct User {
    #[identity]
    id: u64,
    name: RefCell<Strong<str>>,
}

struct Users {
    names: Arena<str>,
    users: Arena<User>,
}
impl Users {
    fn new() -> Self {
        Self {
            names: Arena::new(),
            users: Arena::new(),
        }
    }

    fn insert_or_update(&'_ self, id: u64, name: &str) -> Ref<'_, User> {
        self.users
            .entry::<u64>(&id)
            .and_modify(|user| {
                let name = self.names.intern(name).strong();
                user.name.replace(name);
            })
            .or_insert_with(|| {
                let name = self.names.intern(name).strong();
                User { id, name: RefCell::new(name) }
            })
    }
}
```
