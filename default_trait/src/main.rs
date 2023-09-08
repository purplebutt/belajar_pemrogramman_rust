#![allow(unused)]

#[derive(Debug)]
enum Race {
    Humanoid,
    Beast,
    Spectre,
    Elemental,
    Org,
    Fairy,
    Other
}


#[derive(Debug)]
struct Hero {
    name: String,
    race: Race,
    health: u32,
    regen: u32,
    physical_def: u32,
    magic_resist: u32,    // magical resistant
    ability_pwr: u32,
    attack_damage: u32,
    move_speed: u32,
    attack_speed: u32
}

impl Default for Hero {
    fn default() -> Self {
        Self { 
            name: "no_name".to_string(), 
            race: Race::Other, 
            health: 700, 
            regen: 5, 
            physical_def: 8, 
            magic_resist: 8, 
            ability_pwr: 90, 
            attack_damage: 75, 
            move_speed: 12, 
            attack_speed: 20 
        }
    }
}

// impl Hero {
//     fn new(name: &str, race: Race, health: u32) -> Self {
//         Self { 
//             name: name.to_string(), race, health, 
//             // default value for others fields
//             regen: 5, physical_def: 8, magic_resist: 8, ability_pwr: 90, 
//             attack_damage: 75, move_speed: 12, attack_speed: 20
//         }
//     }
// }

// impl Hero {
//     fn new(name: &str, race: Race, health: u32, regen: u32, 
//         physical_def: u32, magic_resist: u32, ability_pwr: u32, 
//         attack_damage: u32, move_speed: u32, attack_speed: u32
//     ) -> Self {
//         Self { 
//             name: name.to_string(), 
//             race, health, regen, physical_def, magic_resist, 
//             ability_pwr, attack_damage, move_speed, attack_speed 
//         }
//     }
// }

impl Hero {
    fn new(name: &str, race: Race, health: u32) -> Self {
        Self { 
            name: name.to_string(),
            race,
            health,
            ..Default::default()    // set other fields to default value
        }
    }
}


fn main() {
    // setting untuk field name, race dan health
    let seraphine = Hero {
        name: "Seraphine".to_string(), 
        race: Race::Fairy, 
        health: 500, 
        ..Default::default()

        // regen: 10, 
        // physical_def: 30, 
        // magic_resist: 23, 
        // ability_pwr: 50, 
        // attack_damage: 110,
        // move_speed: 10, 
        // attack_speed: 8
    };

    // setting untuk field name dan ability_pwr
    let lulu = Hero {
        name: "Lulu".to_string(),
        ability_pwr: 240,
        ..Default::default()
    };

    // membuat instance dengan metode new()
    let mundo = Hero::new("Mundo", Race::Org, 800);

    println!("{seraphine:?}");
    println!("{lulu:?}");
    println!("{mundo:?}");
}

