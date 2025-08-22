use crate::game::Map;

pub struct World {
    pub map: Map,
}

impl World {
    pub fn new() -> Self {
        let map = Map::new();
        Self { map }
    }
}
