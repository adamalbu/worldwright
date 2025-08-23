#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub current_room: petgraph::prelude::NodeIndex,
}
