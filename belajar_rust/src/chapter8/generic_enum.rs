
pub enum Status<T> {
    BEKERJA {perusahaan: String, gaji: T},
    PENSIUNAN (T),
    MAHASISWA,
}

pub fn test1() {
    let ganjar = Status::BEKERJA { 
        perusahaan: "PT.123".to_string(), 
        gaji: "9_375_000"
    };

    let karni = Status::PENSIUNAN("2,350,000");
    // let anies= Status::MAHASISWA;            // concrete type harus di definisikan
    // let anies= Status::<&str>::MAHASISWA;    // turbofish syntax
    let anies: Status<&str>= Status::MAHASISWA;

    // simpan data workers menggunakan vector
    let workers = vec![ganjar, karni, anies];

    // looping untuk setiap worker
    for (idx, w) in workers.iter().enumerate() {
        match w {
            Status::MAHASISWA => println!("worker {} -> Belum berpenghasilan", idx),
            Status::BEKERJA { perusahaan: _, gaji } | Status::PENSIUNAN(gaji) => {
                println!("worker {} -> Gaji/Uang pensiun: {}", idx, gaji)
            }
        }
    }
}
