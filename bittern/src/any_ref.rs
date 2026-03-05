use crate::Arena;

/// Trait for references to an item in an arena.
pub trait AnyRef<T: ?Sized> {
    /// Gets the underlying raw pointer of a reference
    fn as_ptr(&self) -> *const T;

    /// Returns `true` if this item is allocated in the specified arena.
    fn owned_by(&self, arena: &Arena<T>) -> bool;

    /// Returns `true` if two references point to the same item, using a pointer equality check.
    ///
    /// Equivalent to [`PartialEq::eq`], but `is` may be preferred for semantic clarity.
    #[inline]
    fn is<O: AnyRef<T>>(&self, other: &O) -> bool {
        core::ptr::eq(self.as_ptr(), other.as_ptr())
    }

    /// Returns `true` if two references point to different items, using a pointer equality check.
    /// Equivalent to `!self.is(other)`.
    ///
    /// Equivalent to [`PartialEq::ne`], but `is_not` may be preferred for semantic clarity.
    #[inline]
    fn is_not<O: AnyRef<T>>(&self, other: &O) -> bool {
        !self.is(other)
    }
}


macro_rules! inherent_ref_methods {
    (<$T:ident>) => {
        /// Returns `true` if two references point to the same item, using a pointer equality check.
        ///
        /// Equivalent to [`PartialEq::eq`], but `is` may be preferred for semantic clarity.
        #[inline]
        pub fn is<O: crate::AnyRef<$T>>(&self, other: &O) -> bool {
            crate::AnyRef::is(self, other)
        }

        /// Returns `true` if two references point to different items, using a pointer equality check.
        /// Equivalent to `!self.is(other)`.
        ///
        /// Equivalent to [`PartialEq::ne`], but `is_not` may be preferred for semantic clarity.
        #[inline]
        pub fn is_not<O: crate::AnyRef<$T>>(&self, other: &O) -> bool {
            crate::AnyRef::is_not(self, other)
        }
    };
}
pub(crate) use inherent_ref_methods;