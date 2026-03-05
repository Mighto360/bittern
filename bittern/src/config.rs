/// Config builder for the arena
pub struct ArenaConfig {
    /// Whether `Drop::drop` will be called on all items when the arena is dropped.
    ///
    /// Defaults to `true`.
    pub drop_items: bool,
}
impl ArenaConfig {
    /// Whether `Drop::drop` will be called on all items when the arena is dropped.
    ///
    /// Defaults to `true`.
    pub fn drop_items(mut self, drop_items: bool) -> Self {
        self.drop_items = drop_items;
        self
    }
}
impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            drop_items: true,
        }
    }
}
