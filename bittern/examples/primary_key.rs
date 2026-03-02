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
