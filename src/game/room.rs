#[derive(Clone, Debug)]
/// A struct representing a room in the [`Map`](crate::Map).
///
/// Each `Room` has a description.
pub struct Room {
    /// A description of the room.
    pub description: String,
}

impl Room {
    /// Creates a new `Room` with the given `description`.
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
