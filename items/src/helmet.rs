pub struct Helmet {
    pub name: &'static str,
    pub defense: u8,
}

impl Helmet {
    pub const fn new(name: &'static str, defense: u8) -> Helmet {
        Helmet { name, defense }
    }
}

pub const BARE_HELMET: Helmet = Helmet::new("Empty", 0);
