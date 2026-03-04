use core::ptr::NonNull;
use crate::internal::index::HashIndex;

pub(crate) struct IndexRefIter<'a, T: ?Sized> {
    inner: IndexIter<'a, T>,
    _guard: core::cell::Ref<'a, HashIndex<T>>
}
impl<'a, T: ?Sized> IndexRefIter<'a, T> {
    pub(crate) fn new(index: core::cell::Ref<'a, HashIndex<T>>) -> Self {
        let inner = unsafe {
            // SAFETY: extending the lifetime is safe because of the field declaration order.
            // `inner` will be dropped before `_guard`.
            let iter = index.iter();
            core::mem::transmute::<IndexIter<'_, T>, IndexIter<'a, T>>(iter)
        };
        Self {
            inner,
            _guard: index,
        }
    }
}
impl<'a, T: ?Sized> Iterator for IndexRefIter<'a, T> {
    type Item = &'a NonNull<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub(crate) struct IndexIter<'a, T: ?Sized> {
    inner: hashbrown::hash_table::Iter<'a, NonNull<T>>
}
impl<'a, T: ?Sized> IndexIter<'a, T> {
    pub(crate) fn new(inner: hashbrown::hash_table::Iter<'a, NonNull<T>>) -> Self {
        Self { inner }
    }
}
impl<'a, T: ?Sized> Iterator for IndexIter<'a, T> {
    type Item = &'a NonNull<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
