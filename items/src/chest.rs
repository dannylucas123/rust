pub struct Chest {
    pub name: &'static str,
    pub defense: u8,
    pub id: u8,
}

impl Chest {
    pub const fn new(name: &'static str, defense: u8, id: u8) -> Chest {
        Chest { name, defense, id }
    }
}

pub const BARE_CHEST: Chest = Chest::new("Empty", 0, 0);
