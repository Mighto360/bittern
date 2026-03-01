use crate::{Arena, Identity, Item};

pub struct EntryOccupied<V: ?Sized> {
    pub(crate) item: Item<V>,
}

pub struct EntryVacant<V: ?Sized> {
    pub(crate) arena: Arena<V>
}

/// Utility for handling an identity entry which may or may not exist
pub enum Entry<V: ?Sized> {
    Occupied(EntryOccupied<V>),
    Vacant(EntryVacant<V>),
}
impl<V: ?Sized> Entry<V> {
    pub fn item(self) -> Option<Item<V>> {
        match self {
            Self::Occupied(entry) => Some(entry.item),
            Self::Vacant(_) => None,
        }
    }
}
impl<V: Identity> Entry<V> {
    pub fn or_insert(self, default: V) -> Item<V> {
        match self {
            Self::Occupied(entry) => entry.item,
            Self::Vacant(entry) => entry.arena.intern_owned(default),
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> Item<V>
    where F: FnOnce() -> V
    {
        match self {
            Self::Occupied(entry) => entry.item,
            Self::Vacant(entry) => entry.arena.intern_owned(default()),
        }
    }
}
impl<V: Identity + Default> Entry<V> {
    pub fn or_default(self) -> Item<V> {
        self.or_insert_with(Default::default)
    }
}
