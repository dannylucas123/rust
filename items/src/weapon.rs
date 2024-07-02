pub struct Weapon {
    pub name: &'static str,
    pub attack: u8,
}

impl Weapon {
    pub const fn new(name: &'static str, attack: u8) -> Weapon {
        Weapon { name, attack }
    }
}

pub const FISTS: Weapon = Weapon::new("Fists", 1);
pub const NOZARASHI: Weapon = Weapon::new("Nozarashi", 250);
