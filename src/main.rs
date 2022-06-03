#![allow(dead_code)]
#![allow(non_snake_case)]

use ndm::Dice;

fn main() {
    let mut person_a = Enemy::new(); 
    let mut person_b = Enemy::new(); 
    while person_a.alive() && person_b.alive() {
        person_a.attack(&mut person_b);
        println!("Person A: {:?}", person_a);
        println!("Person B: {:?}", person_b);
    }
}

#[derive(Debug)]
struct StatBlock {
    stamina: i16,
    stamina_cap: i16,
    luck: i16,
    luck_cap: i16,
    skill: i16,
    alive: bool,
}


impl StatBlock {
    fn roll_new() -> StatBlock {
        let stamina_roll: i16 = Dice::new(2, 6).unwrap().total() as i16;
        let luck_roll: i16 = Dice::new(1, 6).unwrap().total() as i16;
        let skill_roll: i16 = Dice::new(1, 3).unwrap().total() as i16;

        StatBlock {
            stamina: stamina_roll + 12,
            stamina_cap: stamina_roll + 12,
            luck: luck_roll + 6,
            luck_cap: luck_roll + 6,
            skill: skill_roll + 3,
            alive: true,
        }
    }

    fn print_block(&self) {
        println!("{:?}", self);
    }

    fn damage(&mut self, attack: i16) {
        // Reduce stamina
        self.stamina -= attack;
        // Check if stamina is zero or below
        if self.stamina <= 0 {
            self.stamina = 0;
            self.alive = false;
        }
    }

    fn heal(&mut self, restore: i16) {
        // check for negitive and non healing
        if restore <= 0 {
            // Do nothing
        } else {
            self.stamina += restore;
            // Check for over healing
            if self.stamina > self.stamina_cap {
                self.stamina = self.stamina_cap;
            }
        }
    }

    fn reduce_luck(&mut self, loss: i16) {
        // Check for negitive or non loss
        if loss <= 0 {
            // Do nothing
        } else {
            // reduce luck value
            self.luck -= loss;
            // check if luck is below minimum 
            if self.luck < 0 {
                self.luck = 0;
            }
        }
    }

    fn restore_luck(&mut self, gain: i16) {
        // Check for negitive or non gain
        if gain <= 0 {
            // Do nothing
        } else {
            // increase luck value
            self.luck += gain;
            // check if luck is above maximum
            if self.luck > self.luck_cap {
                self.luck = self.luck_cap;
            }
        }
    }
}

#[derive(Debug)]
struct Enemy {
    stats: StatBlock,
    initiative: i16,
    armour: i16,
    weapon: Weapon,
}

impl Enemy {
    fn new() -> Enemy {
        Enemy {
            stats: StatBlock::roll_new(),
            initiative: 1,
            armour: 0,
            weapon: Weapon::knife(),
        }
    }

    fn new_goblin() -> Enemy {
        let mut stats = StatBlock::roll_new();
        stats.skill = 3;
        stats.stamina = 4;
        stats.stamina_cap = 4;
        
        Enemy {
            stats: stats,
            initiative: 1,
            armour: 1,
            weapon: Weapon::axe(),
        }
    }

    fn damage(&mut self, attack_roll: i16, weapon: &Weapon) {
        let ajusted_roll = attack_roll - self.armour;
        self.stats.damage(weapon.damage(ajusted_roll));
    }

    fn attack(&self, target: &mut Enemy) {
        let roll:i16 = Dice::new(1, 6).unwrap().total() as i16;
        println!("I rolled a {}!", roll);
        target.damage(roll, &self.weapon);
    }

    fn alive(&self) -> bool {
        self.stats.alive
    }
}


#[derive(Debug)]
struct Weapon {
    name: String,
    damage: [i16;7],
}

impl Weapon {
    fn damage(&self, roll: i16) -> i16 {
        // Check if index is out of bounds
        if roll <= 0 {
            // no damage for small rolls
            return 0;
        } else if roll >= 7 {
            // Max damage for large rolls
            return self.damage[6];
        }
        // Return damage value from damage table
        self.damage[(roll-1) as usize]
    }

    fn sword() -> Weapon {
        Weapon {
            name: "Sword".to_string(),
            damage: [4, 6, 6, 6, 6, 8, 10],
        }
    }

    fn axe() -> Weapon {
        Weapon {
            name: "Axe".to_string(),
            damage: [2, 2, 6, 6, 8, 10, 12],
        }
    }

    fn knife() -> Weapon {
        Weapon {
            name: "Knife".to_string(),
            damage: [2, 2, 2, 2, 4, 8, 10],
        }
    }

    fn unarmed() -> Weapon {
        Weapon {
            name: "Unarmed".to_string(),
            damage: [1, 1, 1, 2, 2, 3, 4],
        }
    }
}

/*
###################
# Mien Table code #
###################
*/

struct Mood (String);

struct Mien {
    moods: [Mood;6],
}

impl Mien {
    fn goblin_table() -> Mien {
        Mien {
            moods: [
                Mood ("Curious".to_string()),
                Mood ("Dismissive".to_string()),
                Mood ("Preoccupied".to_string()),
                Mood ("Gossipy".to_string()),
                Mood ("Overly Friendly".to_string()),
                Mood ("Paranoid".to_string()),
            ]
        }
    }
}