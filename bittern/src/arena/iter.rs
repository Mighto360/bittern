use crate::{Arena, Ref};
use crate::internal::iter::IndexRefIter;

/// Iterator over all items in an [`Arena`], in arbitrary order.
pub struct ArenaIter<'a, T: ?Sized> {
    inner: IndexRefIter<'a, T>,
    arena: &'a Arena<T>,
}
impl<'a, T: ?Sized> ArenaIter<'a, T> {
    pub(crate) fn new(inner: IndexRefIter<'a, T>, arena: &'a Arena<T>) -> Self {
        Self { inner, arena }
    }
}

impl<'a, T: ?Sized> Iterator for ArenaIter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(ptr) => Some(Ref::new(*ptr, self.arena)),
            None => None,
        }
    }
}
