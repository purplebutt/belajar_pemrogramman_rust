use std::cmp::PartialEq;
use std::cmp::PartialOrd;


struct Mobil {
    nomor: String,
    tahun: u32
}
impl Mobil {
    pub fn new(nomor: String, tahun: u32) -> Self {
        Self { nomor, tahun }
    }
}
impl PartialEq for Mobil {
    fn eq(&self, other: &Self) -> bool {
        self.nomor == other.nomor
    }
}
impl PartialOrd for Mobil {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.nomor < other.nomor {
            return Some(std::cmp::Ordering::Less)
        }
        else if self.nomor > other.nomor {
            return Some(std::cmp::Ordering::Greater)
        }
        else {
            return Some(std::cmp::Ordering::Equal)
        }
    }
}

pub fn demo() {
    let m1 = Mobil::new("DA234".into(), 2023);
    let m2 = Mobil::new("DB775".into(), 2010);
    let m3 = Mobil::new("DC279".into(), 2016);
    
    let mycar = Mobil::new("DB775".into(), 2016);
    let a = m1 < mycar;
    let b = m2 <= mycar;
    let c = m3 != mycar;

    println!("{a}, {b}, {c}");
}
