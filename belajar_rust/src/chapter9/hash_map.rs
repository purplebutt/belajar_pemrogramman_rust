
pub struct Akun {
    nomor: u32,
    nama: String,
}

impl Akun {
    pub fn new(nomor: u32, nama: &str) -> Self {
        Self {
            nomor, nama: String::from(nama)
        }
    }

    pub fn set_nama(&mut self, nama_baru: &str) {
        self.nama = String::from(nama_baru)
    }

    // tidak ada implementasi untuk set_nomor karena
    // nomor akun tidak boleh di modifikasi

    pub fn get_nama(&self) -> &str {
        &self.nama
    }

    pub fn get_nama_to_owned(&self) -> String {
        (&self.nama).to_string()
    }

    pub fn get_nomor(&self) -> u32 {
        self.nomor  // primitive type implements Copy traits.
    }

}


use std::{collections::HashMap, str::FromStr};

// tuple struct dengan tipe data HashMap<u32, Akun>
pub struct COA(HashMap<u32, Akun>);

fn helper_get_error(nomor: u32) -> Result<(), String> {
    Err(format!("Akun dengan nomor {} sudah ada.
        \nJika ingin mengganti akun, gunakan fungsi tambah_akun_unchecked.", 
        nomor))
}

impl COA {
    pub fn new() -> Self {
        Self(HashMap::<u32, Akun>::new())
    }

    pub fn tambah_akun_unchecked(&mut self, akun: Akun) {
        self.0.insert(akun.nomor, akun);
    }
    
    pub fn tambah_akun(&mut self, akun: Akun) -> Result<(), String> {
        if !self.0.contains_key(&akun.nomor) {
            self.tambah_akun_unchecked(akun);
            Ok(())
        }
        else {
            helper_get_error(akun.nomor)
        }
    }

    pub fn tambah_akun_tuple(&mut self, d: (u32, &str)) -> Result<(), String>  {
        if !self.0.contains_key(&d.0) {
            self.tambah_akun_unchecked(
                Akun::new(d.0, d.1)
            );
            Ok(())
        }
        else {
            helper_get_error(d.0)
        }
    }

    pub fn edit_akun(&mut self, nomor: u32) -> Result<&mut Akun, String> {
        self.0.get_mut(&nomor).ok_or(
            format!("Tidak ada akun dengan nomor {}", nomor)
        )
    }

    pub fn hapus_akun(&mut self, nomor: u32) -> Option<Akun> {
        self.0.remove(&nomor)
    }

    pub fn print_one(&self, nomor: u32) {
        if let Some(akun) = self.0.get(&nomor) {
            print!("{}\t{}", akun.nomor, akun.nama)
        }
        else {
            print!("Tidak ada akun dengan nomor {}", nomor)
        }
    }

    pub fn buat_report(&self) {
        for (no_urut, (nomor, akun)) in self.0.iter().enumerate() {
            println!("{}.\t{}\t{}", no_urut, nomor, akun.nama)
        }
    }
}



pub fn hashmap_other_functions() {
    let mut superhero: HashMap<&str, String> = HashMap::new();

    superhero.insert("peter", "Spiderman".into());
    superhero.insert("bruce", "Batman".into());
    superhero.insert("wiro", "Pendekar 212".into());
    superhero.insert("panji", "Manusia Millenium".into());
    superhero.insert("wanda", "Scarlet Witch".into());
    superhero.insert("badra", "Sibuta Dari Goa Hantu".into());

    let semua_keys: Vec<&&str> = superhero.keys().collect();
    let semua_values: Vec<&String> = superhero.values().collect();

    let jumlah_hero = superhero.len();
    let ukuran_hashmap = superhero.capacity();
    let is_no_data = superhero.is_empty();
    superhero.clear();   // hapus semua data


    println!("{}", superhero.get("wiro").unwrap());

    // let old_value = superhero.get("wiro").unwrap();
    // let mut new_value = String::from_str(old_value).unwrap();
    // new_value.push_str(" Wirosableng");
    // superhero.insert("wiro", new_value);

    // potongan kode pada baris ke-127 s/d ke 130 dapat di sederhanakan
    // dengan menggunakan attribute and_modify seperti berikut ini.
    superhero.entry("wiro").and_modify(|o| { *o += " Wirosableng"; });

    println!("{}", superhero.get("wiro").unwrap());

}