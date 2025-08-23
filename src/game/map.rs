use crate::{
    Exit,
    game::{Direction, ExitType, Room},
};
use petgraph::prelude::{Graph, NodeIndex};

/// Indicates whether an exit is leading away from or towards a node.
#[derive(Clone, Copy, Debug)]
pub enum ExitWay {
    /// The exit is leading away from the node.
    From,
    /// The exit is going towards the node.
    To,
}

/// A map of [`Room`]s connected by [`ExitType`]s and [`Direction`]s.
///
/// A `Map` is a graph where nodes are [`Room`]s and edges are tuples of [`Direction`]s and [`ExitType`]s.
///
/// # Examples
/// ```
/// use worldwright::{Direction, ExitType, Room};
/// use worldwright::exit_types::Door;
///
/// let mut map = worldwright::Map::new();
/// let foyer_id = map.new_room("You are in the dusty foyer of an old manor.".into());
/// let exit = Door::new_with_name(false, "heavy wooden door".into());
/// let grand_hall_id = map.new_room_in_direction(
///    foyer_id,
///    Direction::North,
///    Box::new(exit),
///    "You step into the magnificent Grand Hall.".into(),
/// );
/// ```
#[derive(Debug)]
pub struct Map {
    /// The underlying graph structure of the map.
    pub graph: Graph<Room, Exit>,
}

impl Map {
    /// Creates a new, empty `Map`.
    pub fn new() -> Self {
        let map = Graph::new();
        Self { graph: map }
    }

    /// Creates a new [`Room`] in the `Map`.
    ///
    /// Creates a new [`Room`] with the given `description`, adds it to the map, and returns the `NodeIndex` of the new room.
    ///
    /// # Examples
    /// ```
    /// use worldwright::Map;
    ///
    /// let mut map = Map::new();
    /// assert!(map.graph.node_count() == 0);
    /// let room_id = map.new_room("You are in a small, cozy room.".into());
    /// assert!(map.graph.node_count() == 1);
    /// ```
    pub fn new_room(&mut self, room_description: String) -> NodeIndex {
        let room = Room::new(room_description);
        self.graph.add_node(room)
    }

    /// Adds an existing [`Room`] to the `Map`.
    ///
    /// Adds the given room to the map and returns its `NodeIndex`.
    ///
    /// # Examples
    /// ```
    /// use worldwright::{Map, Room};
    ///
    /// let mut map = Map::new();
    /// let room = Room::new("You are in a bright, sunny room.".into());
    /// assert!(map.graph.node_count() == 0);
    /// let room_id = map.add_room(room);
    /// assert!(map.graph.node_count() == 1);
    pub fn add_room(&mut self, room: Room) -> NodeIndex {
        self.graph.add_node(room)
    }

    /// Creates and connects a new [`Room`] in a specified [`Direction`] from an existing [`Room`].
    ///
    /// Creates a new room with the given `description`, adds it to the `Map`, and connects
    /// it to the specified existing [`Room`] in the given [`Direction`] using the provided [`Direction`] as the other [`Room`]'s exit.
    ///
    /// # Examples
    /// ```
    /// use worldwright::{Direction, ExitType, Map};
    /// use worldwright::exit_types::RegularExit;
    ///
    /// let mut map = Map::new();
    /// let foyer_id = map.new_room("You are in the dusty foyer of an old manor.".into());
    /// assert!(map.graph.node_count() == 1);
    /// let exit = RegularExit;
    /// let grand_hall_id = map.new_room_in_direction(
    ///    foyer_id,
    ///    Direction::North,
    ///    Box::new(exit),
    ///   "You step into the magnificent Grand Hall.".into(),
    /// );
    /// assert!(map.graph.node_count() == 2);
    /// # // TODO: Add assertion to check the edge between the two rooms.
    /// ```
    pub fn new_room_in_direction(
        &mut self,
        from: NodeIndex,
        direction: Direction,
        exit: Box<dyn ExitType>,
        room_description: String,
    ) -> NodeIndex {
        let to = self.new_room(room_description);
        self.graph.add_edge(from, to, Exit::new(direction, exit));
        to
    }

    /// Connects two existing [`Room`]s in the `Map`.
    ///
    /// Connects the [`Room`] identified by `from` to the [`Room`] identified by `to` in the specified `Direction`
    pub fn connect_rooms(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        direction: Direction,
        exit: Box<dyn ExitType>,
    ) {
        self.graph.add_edge(from, to, Exit::new(direction, exit));
    }

    /// Retrieves all [`Exit`]s connected to a given [`Room`], along with their [`Direction`].
    ///
    /// Returns a vector of tuples containing references to the [`Exit`] and an [`ExitWay`] indicating whether the exit is going away from or to the node.
    ///
    /// **Important**: The direction of the exit is determined by the direction it was added to the graph, NOT relative to this node.
    /// To get the relative direction, you can use the [`get_relative_direction`](Map::get_relative_direction) method.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::RegularExit;
    /// use worldwright::{Direction, ExitType, Map};
    ///
    /// let mut map = Map::new();
    /// let central_room = map.new_room("You are in the central room.".into());
    ///
    /// let upper_room = map.new_room_in_direction(
    ///     central_room,
    ///     Direction::North,
    ///     Box::new(RegularExit),
    ///     "You are in the upper room.".into(),
    /// );
    /// assert_eq!(map.get_exits(central_room).len(), 1);
    /// let (central_room_first_exit, _) = map.get_exits(central_room)[0];
    /// assert_eq!(central_room_first_exit.direction, Direction::North);
    ///
    /// let lower_room = map.new_room("You are in the lower room.".into());
    /// map.connect_rooms(
    ///     lower_room,
    ///     central_room,
    ///     Direction::North,
    ///     Box::new(RegularExit),
    /// );
    /// assert_eq!(map.get_exits(central_room).len(), 2);
    /// let (central_room_second_exit, central_room_second_exit_way) = map.get_exits(central_room)[1];
    /// // It should still be North, because the direction is determined by how it was added to the graph, not relative to this node.
    /// assert_eq!(central_room_second_exit.direction, Direction::North);
    ///
    /// // To get the relative direction, you can use the get_relative_direction method.
    /// let relative_direction =
    ///     map.get_relative_direction(central_room_second_exit, central_room_second_exit_way);
    /// assert_eq!(relative_direction, Direction::South);
    pub fn get_exits(&self, room_id: NodeIndex) -> Vec<(&Exit, ExitWay)> {
        let edges_from = self
            .graph
            .edges_directed(room_id, petgraph::Direction::Outgoing);
        let edges_to = self
            .graph
            .edges_directed(room_id, petgraph::Direction::Incoming);

        let mut exits = Vec::new();

        for edge in edges_from {
            exits.push((edge.weight(), ExitWay::From));
        }

        for edge in edges_to {
            exits.push((edge.weight(), ExitWay::To));
        }

        exits
    }

    /// Gets the relative [`Direction`] of an [`Exit`] based on the specified [`ExitWay`].
    pub fn get_relative_direction(&self, exit: &Exit, exit_way: ExitWay) -> Direction {
        match exit_way {
            ExitWay::From => exit.direction,
            ExitWay::To => exit.direction.opposite(),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
