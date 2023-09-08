pub trait Defend<T> {
    fn print(&mut self, damage: T);
}

pub struct Hero {
    name: String,
    health: u32,
    defend: u32,
}

#[allow(unused)]
#[derive(Debug)]
pub enum MagicElement {
    Fire(u32),
    Water(u32),
    Earth(u32),
    Wind(u32)
}

impl MagicElement {
    fn get_damage(&self) -> &u32 {
        match self {
            Self::Fire(dmg) => dmg,
            Self::Water(dmg) => dmg,
            Self::Earth(dmg) => dmg,
            Self::Wind(dmg) => dmg,
        }
    }
}

impl Hero {
    pub fn new(name: &str, health: u32, defend: u32) -> Self {
        Self { 
            name: name.to_string(), 
            health,
            defend
        }
    }
}

impl Defend<MagicElement> for Hero {
    fn print(&mut self, damage: MagicElement) {
        println!("{} is defending {:?} damage", self.name, damage);
        let net_damage = damage.get_damage() - self.defend;
        self.health -= net_damage;
        println!("{} is taking {} magic damage. Current health is {}", 
            self.name, net_damage, self.health);
    }
}

impl Defend<u32> for Hero {
    fn print(&mut self, damage: u32) {
        println!("{} is defending physical damage", self.name);
        let net_damage = damage - self.defend;
        self.health -= net_damage;
        println!("{} is taking {} physical damage. Current health is {}", 
            self.name, net_damage, self.health);
    }
}
