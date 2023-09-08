#[derive(Debug)]
pub enum TipeUang {
    Kertas,
    Koin
}

pub struct Uang {
    nilai: f32,
    tipe: TipeUang
}


pub fn pattern_match_demo() {
    let type_u = TipeUang::Kertas;
    let uang = Uang { nilai: 25_000f32, tipe: type_u };

    display(uang);
}

pub fn display(u: Uang) {
    match u {
        Uang { nilai, tipe } if nilai > 50_000f32 => {
            println!("Uang {:?} > dari 50 ribu", tipe)
        },
        Uang { nilai, tipe } => { 
            match tipe {
                TipeUang::Kertas => println!("Uang kertas bernilai: {}", nilai),
                TipeUang::Koin => println!("Uang kertas bernilai: {}", nilai)
            }
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    score: Option<u32>
}
impl Player {
    fn new_instance(name: &str) -> Self {
        Self { name: name.into(), score: None }
    }
    fn set_score(&mut self, score: u32) {
        self.score = Some(score)
    }
}

pub fn option_demo() {
    let mut joko = Player::new_instance("Jokowido");
    println!("{:?}", joko);

    // update skor
    joko.set_score(30);
    println!("Nama: {}, Skor: {}", joko.name, joko.score.unwrap());
}
