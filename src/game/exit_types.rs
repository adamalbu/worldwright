use crate::game::{Direction, Exit};
use crate::starts_with_vowel;

pub struct RegularExit;

impl Exit for RegularExit {
    fn can_go_through(&self) -> bool {
        true
    }

    fn description(&self, direction: Direction) -> String {
        format!("an exit {}", direction)
    }
}

pub struct NamedExit {
    pub name: String,
}

impl Exit for NamedExit {
    fn can_go_through(&self) -> bool {
        true
    }

    fn description(&self, direction: Direction) -> String {
        format!(
            "a{} {} {}",
            if starts_with_vowel(&self.name) {
                "n"
            } else {
                ""
            },
            self.name,
            direction
        )
    }
}

pub struct Door {
    pub locked: bool,
    pub name: Option<String>,
}

impl Door {
    pub fn new(locked: bool) -> Self {
        Self { locked, name: None }
    }

    pub fn new_with_name(locked: bool, name: String) -> Self {
        Self {
            locked,
            name: Some(name),
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn toggle_lock(&mut self) {
        self.locked = !self.locked;
    }
}

impl Exit for Door {
    fn can_go_through(&self) -> bool {
        !self.locked
    }

    fn description(&self, direction: Direction) -> String {
        if let Some(name) = &self.name {
            format!(
                "a{} {} door {}",
                if starts_with_vowel(name) { "n" } else { "" },
                name,
                direction
            )
        } else {
            format!("a door {}", direction)
        }
    }
}
