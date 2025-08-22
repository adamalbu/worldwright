use crate::game::Direction;

pub trait Exit {
    fn can_go_through(&self) -> bool;
    fn description(&self, direction: Direction) -> String;
}
