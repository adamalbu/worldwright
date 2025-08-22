#[derive(Clone, Debug)]
pub struct Room {
    pub description: String,
}

impl Room {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
