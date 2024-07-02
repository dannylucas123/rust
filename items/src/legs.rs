pub struct Legs {
    pub name: &'static str,
    pub defense: u8,
}

impl Legs {
    pub const fn new(name: &'static str, defense: u8) -> Legs {
        Legs { name, defense }
    }
}

pub const BARE_LEGS: Legs = Legs::new("Empty", 0);
