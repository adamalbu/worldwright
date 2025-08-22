use crate::game::{Direction, Exit, Room};
use petgraph::prelude::{Graph, NodeIndex};

pub struct Map {
    pub map: Graph<Room, (Direction, Box<dyn Exit>)>,
}

impl Map {
    pub fn new() -> Self {
        let map = Graph::new();
        Self { map }
    }

    pub fn new_room(&mut self, room_description: String) -> NodeIndex {
        let room = Room::new(room_description);
        self.map.add_node(room)
    }

    pub fn add_room(&mut self, room: Room) -> NodeIndex {
        self.map.add_node(room)
    }

    pub fn new_room_in_direction(
        &mut self,
        from: NodeIndex,
        direction: Direction,
        exit: Box<dyn Exit>,
        room_description: String,
    ) -> NodeIndex {
        let to = self.new_room(room_description);
        self.map.add_edge(from, to, (direction, exit));
        to
    }

    pub fn connect_rooms(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        direction: Direction,
        exit: Box<dyn Exit>,
    ) {
        self.map.add_edge(from, to, (direction, exit));
    }
}
