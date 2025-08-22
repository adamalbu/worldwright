use crate::game::Direction;

/// An `Exit` represents a passage between two rooms in the map.
///
/// It represents a passage that the player can go through to move from one room to another.
pub trait Exit: std::fmt::Debug {
    /// Checks whether the player can go through this exit.
    fn can_go_through(&self) -> bool;

    /// Provides a description of the exit in a given direction.
    ///
    /// The description should be a short phrase that describes the exit with a direction, such as "a wooden door north" or "an archway south".
    fn description(&self, direction: Direction) -> String;
}
