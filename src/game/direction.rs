#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Represents a cardinal direction.
///
/// It is used in the map to indicate the direction the player can go.
pub enum Direction {
    #[doc(hidden)]
    North,
    #[doc(hidden)]
    East,
    #[doc(hidden)]
    South,
    #[doc(hidden)]
    West,
}

impl Direction {
    /// Returns the opposite car direction.
    ///
    /// This method takes a [Direction] and returns the [`Direction`] it is opposite to.
    /// # Examples
    /// ```
    /// use worldwright::Direction;
    ///
    /// let north = Direction::North;
    /// assert_eq!(north.opposite(), Direction::South);
    ///
    /// let east = Direction::East;
    /// assert_eq!(east.opposite(), Direction::West);
    /// ```
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        };
        write!(f, "{s}")
    }
}
