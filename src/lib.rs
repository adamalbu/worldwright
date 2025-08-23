#![warn(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::all
)]
#![allow(dead_code)]
//! This crate aims to make it easier to create interactive fiction games in Rust.
//! It is inspired by [Inform 7](https://ganelson.github.io/inform-website/).

const VOWELS: &str = "aeiou";

fn starts_with_vowel(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        VOWELS.contains(first_char)
    } else {
        false
    }
}

pub use world::World;

/// The map module contains all the structures and functions related to the game map, including [`Room`](crate::map::Room)s and [`Exit`](crate::map::Exit)s.
///
/// The map module contains everything related to managing the layout of the game map, including creating and connecting [`Room`](crate::map::Room)s, defining [`Exit`](crate::map::Exit)s between [`Room`](crate::map::Room)s, and navigating the [`Map`](crate::map::Map).
pub mod map;
mod world;

fn main() {
    let mut world = World::new();
    let foyer_id = world.map.new_room("You are in the dusty foyer of an old manor. A grand staircase leads up to a landing, but it's roped off.".into());
    let exit = map::exit_types::Door::new_with_name(false, "heavy wooden door".into());

    let grand_hall_id = world.map.new_room_in_direction(
        foyer_id,
        map::Direction::North,
        Box::new(exit),
        "You step into the magnificent Grand Hall. A roaring fireplace dominates the far wall."
            .into(),
    );

    let exit = map::exit_types::Door::new(true);
    let _library_id = world.map.new_room_in_direction(
        grand_hall_id,
        map::Direction::East,
        Box::new(exit),
        "The air here is thick with the scent of old paper. Shelves filled with forgotten books line the walls. A small, sturdy desk stands in the middle of the room."
            .into(),
    );
}
