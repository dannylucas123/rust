pub enum Location {
    Plains,
    Town,
    Forest,
    Dungeon,
}

impl Location {
    pub fn as_str(&self) -> &str {
        match *self {
            Location::Plains => "Plains",
            Location::Town => "Town",
            Location::Forest => "Forest",
            Location::Dungeon => "Dungeon",
        }
    }
}
