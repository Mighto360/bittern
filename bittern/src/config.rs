/// Config builder for the arena
pub struct ArenaConfig {
    pub drop_items: bool,
}
impl ArenaConfig {
    /// default = true
    /// If true: items will be individually dropped when the arena is dropped
    /// If false: items will be deallocated without calling their Drop::drop method
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
