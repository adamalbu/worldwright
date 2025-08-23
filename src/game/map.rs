use crate::game::{Direction, ExitType, Room};
use petgraph::prelude::{Graph, NodeIndex};

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
    pub graph: Graph<Room, (Direction, Box<dyn ExitType>)>,
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
        self.graph.add_edge(from, to, (direction, exit));
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
        self.graph.add_edge(from, to, (direction, exit));
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
