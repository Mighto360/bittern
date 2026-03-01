// Demonstrates an arena of Users, deduplicated by their `id` field

use bittern::{Identity, Arena, Strong, Ref};

fn main() {
    let users = Users::new();

    let u1 = users.intern_by_id(0, "John Doe");
    let u2 = users.intern_by_id(1, "Jane Doe");
    assert!(u1.is_not(&u2));

    let u3 = users.intern_by_id(0, "JOHN");
    assert!(u1.is(&u3));
    assert_eq!(&*u3.name, "John Doe"); // This is the previously interned value, so the name didn't change
}

#[derive(Identity)]
struct User {
    #[identity]
    id: u64,
    name: Strong<str>,
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

    fn intern_by_id(&'_ self, id: u64, name: &str) -> Ref<'_, User> {
        // Use `entry` instead of `intern`, so we can skip allocating the new name if it won't be used
        self.users
            .entry::<u64>(&id)
            .or_insert_with(|| {
                let name = self.names.intern(name).strong();
                User { id, name }
            })
    }
}
