
struct Hero {
    name: String,
    health: i32,
    class: HeroType
}

#[derive(Debug)]
pub enum HeroType {
    Enchanter,
    Knight(KnightType),
    Mage,
    Tank {
        regen: f32,
        resistance: u16
    },
    Marksman,
}

#[derive(Debug)]
pub enum KnightType {
    Paladin,
    Vanguard,
    OmniKnight,
    DragonKnight
}

pub fn create_hero() {
    let asi = Hero {
        name: "Dewi Asi".into(),  // bisa juga "Arthur".to_string()
        health: 3750,
        class: HeroType::Knight(KnightType::Vanguard)
    };

    let mandala = Hero {
        name: "Mandala".into(),
        health: 2350,
        class: HeroType::Mage
    };

    let wiro = Hero {
        name: "Wiro".to_string(), // atau bisa juga "Kuka".into()
        health: 5010,
        class: HeroType::Tank { regen: 50.0, resistance: 16 }
    };

    if let HeroType::Tank { regen, resistance } = wiro.class {
        println!("{}, {}", regen, resistance)
    }

    println!("{}, hp: {}, type: {:?}", asi.name, asi.health, asi.class);
    println!("{}, hp: {}, type: {:?}", mandala.name, mandala.health, mandala.class);
    println!("{}, hp: {}, type: {:?}", wiro.name, wiro.health, wiro.class);
}


// impl pada struct
enum AttackType {
    Range,
    Melee
}

impl AttackType {
    fn display(&self) -> String {
        match self {
            Self::Range => "Attacking from range".to_string(),
            Self::Melee => "Attacking on enemy".to_string()
        }
    }
}

pub fn enum_impl() {
    let luna = AttackType::Range;
    let gara = AttackType::Melee;

    println!("{}", luna.display());     // will print "Attacking from range"
    println!("{}", gara.display());     // will print "Attacking on enemy"
}