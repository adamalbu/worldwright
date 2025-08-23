use crate::{Direction, ExitType};

/// An `Exit` from a [`Room`](crate::Room) in a given [`Direction`] with a specific [`ExitType`].
#[derive(Debug)]
pub struct Exit {
    /// The [`Direction`] of the `Exit`.
    pub direction: Direction,
    /// The [`ExitType`] defines what type of exit and its behavior.
    pub exit_type: Box<dyn ExitType>,
}

impl Exit {
    /// Creates a new `Exit` with the specified [`Direction`] and [`ExitType`]
    pub fn new(direction: Direction, exit_type: Box<dyn ExitType>) -> Self {
        Self {
            direction,
            exit_type,
        }
    }
}
