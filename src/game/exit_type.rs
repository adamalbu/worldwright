use crate::game::Direction;

/// An `ExitType` represents the type of passage between two [`Room`](crate::Room)s in the [`Map`](crate::Map).
///
/// It represents a the type of passage the player can go through to move from one room to another and the conditions needed for the player to be able to go through an exit.
pub trait ExitType: std::fmt::Debug {
    /// Checks whether the player can go through this exit.
    fn can_go_through(&self) -> bool;

    /// Provides a description of the exit in a given direction.
    ///
    /// The description should be a short phrase that describes the exit with a direction, such as "a wooden door north" or "an archway south".
    fn description(&self, direction: Direction) -> String;
}
