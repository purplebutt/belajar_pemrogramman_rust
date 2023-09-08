
pub struct Domino {
    up: u8, dw: u8, idx: u8
}

impl Domino {
    pub fn new(up: u8, dw: u8, idx: u8) -> Self {
        Self { up, dw, idx }
    }

    pub fn to_text(&self) -> String {
        // akan menghasilkan teks dengan format: "|n:n|"
        let mut s = String::new();
        s.push_str("|");
        s.push_str(self.up.to_string().as_str());
        s.push_str(":");
        s.push_str(self.dw.to_string().as_str());
        s.push_str("|");
        s
    }
}

pub struct Dominos(Vec<Domino>);

impl Dominos {
    pub fn new() -> Self {
        // membuat instance baru vector dengan concrete type Domino
        let mut vector = Vec::<Domino>::new();

        let mut i = 0;          // variabel untuk indeks
        for up in 0..=6 {       // outer looping dari 0..=6
            for dw in 0..=6 {   // inner looping dari 0..=6
                vector.push(        // menambahkan data baru pada vector
                    Domino::new(up, dw, i)  // data baru yang di tambahkan pada vector
                );
                i += 1;         // increment indeks
            }
        }
        Self(vector)    // mengembalikan instance Dominos yang baru di buat sebagai nilai kembalian
    }

    pub fn print_all(&self) {
        for domino in self.0.iter() {
            print!("{}  ", domino.to_text())
        }
        print!("\n");   // tambahkan newline
    }
    
    pub fn add_many(&mut self, data: Vec<Domino>) {
        for domino in data.into_iter() {
            self.0.push(domino)
        }
    }

    pub fn to_owned_vector(data: Vec<&Domino>) -> Self {
        let vector = data.iter()
            .map(|&d| Domino::new(d.up, d.dw, d.idx))
            .collect::<Vec<Domino>>();
        Self(vector)
    }

    pub fn filter(self, criteria: u8) -> Self {
        let filtered = self.0.into_iter()
            .filter(|d| d.up == criteria || d.dw == criteria)
            .collect::<Vec<Domino>>();

        // return Self::to_owned_vector(filtered);
        return Self(filtered);
    }

    #[allow(unused)]
    pub fn show_others_usefull_functions(&mut self) {
        let is_empty: bool = self.0.is_empty();                // cek jika vector kosong
        let capacity = self.0.capacity();               // cek kapasitas/ukuran vector
        let first = self.0.first();           // mengambil item pertama
        let last = self.0.last();             // mengambil item pertama
        let item_at_2 = self.0.get(2); // mengambil item ke-n
        let item_at_2 = &self.0[0];                   // mengambil referensi item ke-n
        let len = self.0.len();                         // mengambil jumlah item
        let pop = self.0.pop();                // mengambil item terakhir
        let removed = self.0.remove(2);         // remove item berdasarkan index
        let reversed = self.0.reverse();                    // reverse urutan item
        self.0.clear();                                        // mengosongkan vector

        // menambahkan data pada vector, pada posisi/index 2.
        self.0.insert(2, Domino::new(3, 4, 2));
    }
}


pub fn test() {
    let mut dominos = Dominos::new();
    println!("\n== Tampilan awal - dengan data default ==");
    dominos.print_all();

    let domino_a = Domino::new(1, 1, 1);
    let domino_b = Domino::new(1, 2, 2);
    let domino_c = Domino::new(2, 1, 3);

    dominos.add_many(vec![domino_a, domino_b, domino_c]);
    println!("\n== Tampilan dengan tambahan data ==");
    dominos.print_all();

    println!("\n== Tampilan dengan filter (hanya data yg memiliki nilai 2) ==");
    dominos.filter(2).print_all();
}


