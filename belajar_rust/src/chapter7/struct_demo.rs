
pub fn attack2( 
    attacker_name: &str, atk_pwr: u16, target_def: u16, luck: u16, 
    crit_dmg: u16, target: &mut u16, target_name: &str) {
    
    let mut dmg = 0u16;
    dmg += ((crit_dmg * luck) + atk_pwr) - target_def;

    println!("{} attacking {}", attacker_name, target_name);
    *target -= dmg;
}

pub fn attack(atk_pwr: u16, target: &mut u16) {
    // println!("{} attacking {}", hero1, hero2)
    *target -= atk_pwr;
}

pub fn hero_fight() {
    // hero1
    let _hero1_name = "Joko".to_string();
    let hero1_atk = 20u16;
    let mut hero1_health = 800u16;

    // hero2
    let _hero2_name = "Roro".to_string();
    let hero2_atk = 30u16;
    let mut hero2_health = 650u16;

    // hero1 attacking hero2
    attack(hero1_atk, &mut hero2_health);

    // hero2 attacking hero1
    attack(hero2_atk, &mut hero1_health);

    println!("hero1_health: {}", hero1_health);
    println!("hero2_health: {}", hero2_health);
}


// definisi struct
struct Ping;                // empty struct

// tuple struct
struct Money(f64);          // tuple struct dengan satu field
struct AtkDef(u16, u16);    // tuple struct dengan dua field


// normal struct
struct Hero {
    name: String,
    atk_dmg: i32,
    // atk_spd: u16,
    def: i32,
    luck: i32,
    crit: i32,
    health: i32,
    alive: bool
}

fn attack3(attacker: &Hero, defender: &mut Hero) {
    let mut dmg = 0i32;
    dmg += ((attacker.crit * attacker.luck) + attacker.atk_dmg) - defender.def;

    println!("{} attacking {}", attacker.name, defender.name);
    defender.health -= dmg;
}

struct Karyawan {
    nama: String,
    gaji: f64
}

impl Karyawan {
    fn new() -> Self {
        Self { nama: "No Name".into(),  gaji: 0. }
    }
    fn from_tuple(t: (&str, f64)) -> Self {
        Self { nama: t.0.into(), gaji: t.1 }
    }
    fn display(&self) -> String {
        format!("Nama: {}, Gaji: {}", self.nama, self.gaji)
    }

    // new added
    fn set_gaji(&mut self, gaji: f64) {
        self.gaji = gaji
    }
    fn take_and_return(self, nama: &str) -> Karyawan {
        Karyawan { nama: nama.into(), gaji: self.gaji * 1.2 }
    }
}
