use crate::{Arena, Identity, Ref};

pub struct EntryOccupied<'a, V: ?Sized> {
    pub(crate) item: Ref<'a, V>,
}

pub struct EntryVacant<'a, V: ?Sized> {
    pub(crate) arena: &'a Arena<V>
}

/// Utility for handling an identity entry which may or may not exist
pub enum Entry<'a, V: ?Sized> {
    Occupied(EntryOccupied<'a, V>),
    Vacant(EntryVacant<'a, V>),
}
impl<'a, V: ?Sized> Entry<'a, V> {
    pub fn get(self) -> Option<Ref<'a, V>> {
        match self {
            Self::Occupied(entry) => Some(entry.item),
            Self::Vacant(_) => None,
        }
    }
}
impl<'a, V: Identity> Entry<'a, V> {
    pub fn or_insert(self, default: V) -> Ref<'a, V> {
        match self {
            Self::Occupied(entry) => entry.item,
            Self::Vacant(entry) => entry.arena.intern_owned(default),
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> Ref<'a, V>
    where F: FnOnce() -> V
    {
        match self {
            Self::Occupied(entry) => entry.item,
            Self::Vacant(entry) => entry.arena.intern_owned(default()),
        }
    }
}
impl<'a, V: Identity + Default> Entry<'a, V> {
    pub fn or_default(self) -> Ref<'a, V> {
        self.or_insert_with(Default::default)
    }
}
