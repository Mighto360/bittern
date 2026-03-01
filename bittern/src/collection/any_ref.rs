use crate::Arena;

pub trait AnyRef<T: ?Sized> {
    fn as_ptr(&self) -> *const T;
    fn owned_by(&self, arena: &Arena<T>) -> bool;
}
