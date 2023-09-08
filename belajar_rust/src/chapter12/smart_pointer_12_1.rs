use std::ops::Deref;


pub struct NilaiKartu(u8);

impl Deref for NilaiKartu {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }    
}

impl Drop for NilaiKartu {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

impl NilaiKartu {
    pub fn new(value: u8) -> Self {
        let this = Self(value);
        let this = this.validate().unwrap();
        this
    }
    fn validate(self) -> Result<Self, String> {
        match self.0 {
            1..=13 => Ok(self),
            _ => Err(format!("Nilai '{}' tidak valid", self.0))
        }
    }
}

#[allow(unused)]    // ignore error untuk variant enum yang tidak di gunakan
pub enum TipeKartu {
    Heart,
    Club,
    Spade,
    Diamond
}

impl std::fmt::Display for TipeKartu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = {
            match self {
                Self::Club => "keriting",
                Self::Diamond => "wajik",
                Self::Heart => "hati",
                Self::Spade => "sekop"
            }
        };
        write!(f, "{}", text)
    }
}

pub struct Kartu {
    pub nilai: NilaiKartu,
    pub tipe: TipeKartu
}

impl Kartu {
    pub fn new(nilai: NilaiKartu, tipe: TipeKartu) -> Self {
        Self { nilai, tipe }
    }
}

impl std::fmt::Display for Kartu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self.nilai {
            11 => write!(f, "J {}", self.tipe),
            12 => write!(f, "Q {}", self.tipe),
            13 => write!(f, "K {}", self.tipe),
            1 => write!(f, "A {}", self.tipe),
            1..=13 => write!(f, "{} {}", *self.nilai, self.tipe),
            _ => write!(f, "nilai invalid '{}'", self.nilai.deref())
        }
    }
}

#[allow(unused)]
pub fn demo_kartu() {

    // let nk_err = NilaiKartu::new(14); // jika di uncomment, program panic.
    // let nk = NilaiKartu::new(8);

    // let p = nk.pow(2); // pow() adalah fungsi u8.
    // println!("{}", p);

    // let hati_2 = Kartu::new(NilaiKartu::new(2), TipeKartu::Heart);
    // // let hati_2 = Kartu::new(NilaiKartu(14), TipeKartu::Heart);

    // let n_11 = NilaiKartu::new(11);
    // let n_13 = NilaiKartu::new(13);
    // let keriting_11 = Kartu::new(n_11, TipeKartu::Club);
    // let keriting_13 = Kartu::new(n_13, TipeKartu::Club);

    // println!("{}", hati_2);
    // println!("{}", keriting_13);


    let mut cards = Vec::<Kartu>::new();
    let sekop_as = Kartu::new(NilaiKartu::new(1), TipeKartu::Spade);
    let hati_10 = Kartu::new(NilaiKartu::new(10), TipeKartu::Heart);
    let wajik_j = Kartu::new(NilaiKartu::new(11), TipeKartu::Diamond);

    // kode berikut seharusnya tidak valid
    // let wajik_14 = Kartu::new(NilaiKartu::new(14), TipeKartu::Diamond);

    {
        let ace = NilaiKartu::new(1);
        println!("{}", ace.0);
        drop(ace);
    }
    
    cards.push(sekop_as);
    cards.push(hati_10);
    cards.push(wajik_j);
    //cards.push(wajik_14);

    for k in cards {
        println!("{}", k)
    }
}

