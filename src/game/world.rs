use crate::game::Map;

/// The entire game world.
///
/// Currently, this only contains a [`Map`], it will be expanded in the future to include other global game state like items and entites.
#[derive(Debug)]
pub struct World {
    /// The [`Map`] of the world.
    pub map: Map,
}

impl World {
    /// Creates a new, empty `World`.
    pub fn new() -> Self {
        let map = Map::new();
        Self { map }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
