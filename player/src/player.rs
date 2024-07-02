use inventory::_i::Inventory;
use items::gear::Gear;

pub struct Player {
    name: String,
    level: u8,
    experience_multiplier: Option<u8>,
    attack: u8,
    defense: u8,
    gear: Gear,
    inventory: Inventory,
}

impl Player {
    pub fn new(name: String, level: u8) -> Player {
        let mut player = Player {
            name,
            level: 0,
            experience_multiplier: None,
            attack: 1,
            defense: 1,
            gear: Gear::new(),
            inventory: Inventory::new(),
        };
        player.set_level(level);
        return player;
    }

    pub fn set_weapon(&mut self) {
        self.gear.equip_weapon();
    }

    pub fn set_level(&mut self, new_level: u8) {
        if new_level <= 100 || new_level == 0 {
            self.level = new_level;
        }
    }

    pub fn set_experience_multiplier(&mut self, multiplier: u8) {
        if multiplier == 0 {
            self.experience_multiplier = None;
        } else if multiplier <= 200 {
            self.experience_multiplier = Some(multiplier);
        } else {
            self.experience_multiplier = Some(200);
        }
    }

    pub fn info(&self) {
        println!(
            "The player {:?} has achieved level {:?}",
            self.name, self.level
        )
    }

    pub fn calculate_stats(&mut self) {
        self.attack = self.gear.weapon.attack;
        println!("Attack Power: {}", self.attack);
    }
}
