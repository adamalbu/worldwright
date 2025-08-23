mod direction;
pub use direction::Direction;

mod exit;
pub use exit::Exit;

mod exit_type;
pub use exit_type::ExitType;

mod map;
pub use map::{ExitWay, Map};

mod room;
pub use room::Room;

/// Types of exits that can be used in a [`Map`] between [`Room`]s.
pub mod exit_types;
