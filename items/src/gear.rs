use crate::chest::{Chest, BARE_CHEST};
use crate::helmet::{Helmet, BARE_HELMET};
use crate::legs::{Legs, BARE_LEGS};
use crate::weapon::{Weapon, FISTS, NOZARASHI};

pub struct Gear {
    pub helmet: Helmet,
    pub chest: Chest,
    pub legs: Legs,
    pub weapon: Weapon,
}

impl Gear {
    pub fn new() -> Gear {
        Gear {
            weapon: FISTS,
            helmet: BARE_HELMET,
            chest: BARE_CHEST,
            legs: BARE_LEGS,
        }
    }

    pub fn equip_weapon(&mut self) {
        self.weapon = NOZARASHI;
    }
}
