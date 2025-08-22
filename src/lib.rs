#![warn(missing_docs, clippy::all)]
#![allow(dead_code)]
//! This is a crate for building text adventure games.

const VOWELS: &str = "aeiou";

fn starts_with_vowel(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        VOWELS.contains(first_char)
    } else {
        false
    }
}

use game::{Direction, World, exit_types};

pub mod game;

fn main() {
    let mut world = World::new();
    let foyer_id = world.map.new_room("You are in the dusty foyer of an old manor. A grand staircase leads up to a landing, but it's roped off.".into());
    let exit = exit_types::Door::new_with_name(false, "heavy wooden door".into());

    let grand_hall_id = world.map.new_room_in_direction(
        foyer_id,
        Direction::North,
        Box::new(exit),
        "You step into the magnificent Grand Hall. A roaring fireplace dominates the far wall."
            .into(),
    );

    let exit = exit_types::Door::new(true);
    let library_id = world.map.new_room_in_direction(
        grand_hall_id,
        Direction::East,
        Box::new(exit),
        "The air here is thick with the scent of old paper. Shelves filled with forgotten books line the walls. A small, sturdy desk stands in the middle of the room."
            .into(),
    );
}
