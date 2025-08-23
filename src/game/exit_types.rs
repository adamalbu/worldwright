use crate::game::{Direction, Exit};
use crate::starts_with_vowel;

/// A regular exit that the player can always go through.
///
/// This exit type represents a simple passage between two rooms without any special conditions or restrictions.
///
/// # Examples
/// ```
/// use worldwright::exit_types::RegularExit;
/// use worldwright::{ Direction, Exit };
///
/// let exit = RegularExit;
/// assert!(exit.can_go_through());
/// assert_eq!(exit.description(Direction::North), "an exit north");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RegularExit;

impl Exit for RegularExit {
    /// Lets the player go through the exit.
    ///
    /// Always returns true, as the player can always go through a regular exit.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::RegularExit;
    /// use worldwright::Exit;
    ///
    /// let exit = RegularExit;
    /// assert!(exit.can_go_through());
    /// ```
    fn can_go_through(&self) -> bool {
        true
    }

    /// Provides a generic description of the exit with a direction.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::RegularExit;
    /// use worldwright::{ Direction, Exit };
    ///
    /// let exit = RegularExit;
    /// assert_eq!(exit.description(Direction::North), "an exit north");
    /// ```
    fn description(&self, direction: Direction) -> String {
        format!("an exit {direction}")
    }
}

/// A named exit that the player can always go through.
///
/// This exit type represents a passage between two rooms with a specific name, such as archway or tunnel.
///
/// # Examples
/// ```
/// use worldwright::exit_types::NamedExit;
/// use worldwright::{ Direction, Exit };
///
/// let exit = NamedExit::new("archway".into());
/// assert!(exit.can_go_through());
/// assert_eq!(exit.description(Direction::East), "an archway east");
#[derive(Clone, Debug)]
pub struct NamedExit {
    /// The name of the exit, such as "archway" or "tunnel".
    pub name: String,
}

impl NamedExit {
    /// Creates a new named exit with the given name.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Exit for NamedExit {
    /// Lets the player go through the exit.
    ///
    /// Always returns true, as the player can always go through a regular exit.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::NamedExit;
    /// use worldwright::Exit;
    ///
    /// let exit = NamedExit::new("archway".into());
    /// assert!(exit.can_go_through());
    /// ```
    fn can_go_through(&self) -> bool {
        true
    }

    /// Provides a description of the exit with its name and a direction.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::NamedExit;
    /// use worldwright::{ Direction, Exit };
    ///
    /// let exit = NamedExit::new("archway".into());
    /// assert_eq!(exit.description(Direction::East), "an archway east");
    /// ```
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

/// A door that can be locked or unlocked.
///
/// This exit type represents a door that can be locked or unlocked, preventing or allowing passage between two rooms.
/// Additionally, the door can have an optional name to provide more description.
///
/// # Examples
/// ```
/// use worldwright::exit_types::Door;
/// use worldwright::{ Direction, Exit };
///
/// let mut door = Door::new_with_name(true, "heavy wooden door".into());
/// assert!(!door.can_go_through());
/// assert_eq!(door.description(Direction::North), "a locked heavy wooden door north");
///
/// door.unlock();
///
/// assert!(door.can_go_through());
#[derive(Clone, Debug)]
pub struct Door {
    /// Indicates whether the door is locked.
    pub locked: bool,
    /// An optional name for the door, such as "heavy wooden door".
    pub name: Option<String>,
}

impl Door {
    /// Creates a new door with the specified locked state and no name.
    pub fn new(locked: bool) -> Self {
        Self { locked, name: None }
    }

    /// Creates a new door with the specified lock state and a name/
    pub fn new_with_name(locked: bool, name: String) -> Self {
        Self {
            locked,
            name: Some(name),
        }
    }

    /// Locks the door, preventing passage.
    ///
    /// Sets the door's locked property to true.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::Door;
    /// use worldwright::{ Exit };
    ///
    /// let mut door = Door::new(false);
    /// assert!(!door.locked);
    /// door.lock();
    /// assert!(door.locked);
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// Unlock the door, allowing passage.
    ///
    /// Sets the door's locked property to false.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::Door;
    /// use worldwright::{ Exit };
    ///
    /// let mut door = Door::new(true);
    /// assert!(door.locked);
    /// door.unlock();
    /// assert!(!door.locked);
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

impl Exit for Door {
    /// Always returns true, as the player can always go through a regular exit.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::NamedExit;
    /// use worldwright::Exit;
    ///
    /// let exit = NamedExit::new("archway".into());
    /// assert!(exit.can_go_through());
    /// ```
    fn can_go_through(&self) -> bool {
        !self.locked
    }

    /// Provides a description of the door with its name (if any), if it is locked, and a direction.
    ///
    /// # Examples
    /// ```
    /// use worldwright::exit_types::Door;
    /// use worldwright::{ Direction, Exit };
    ///
    /// let mut door = Door::new_with_name(false, "antique wooden door".into());
    /// assert_eq!(door.description(Direction::West), "an antique wooden door west");
    /// door.lock();
    /// assert_eq!(door.description(Direction::West), "a locked antique wooden door west");
    ///
    /// let mut unnamed_door = Door::new(false);
    /// assert_eq!(unnamed_door.description(Direction::South), "a door south");
    /// unnamed_door.lock();
    /// assert_eq!(unnamed_door.description(Direction::South), "a locked door south");
    /// ```
    fn description(&self, direction: Direction) -> String {
        if let Some(name) = &self.name {
            format!(
                "a{}{}{name} {direction}",
                if starts_with_vowel(name) && !self.locked {
                    "n"
                } else {
                    ""
                },
                if self.locked { " locked " } else { " " }
            )
        } else {
            format!(
                "a{}door {direction}",
                if self.locked { " locked " } else { " " }
            )
        }
    }
}
