pub fn variabel_definition() {
    // variabel tanpa nilai awal dan tanpa tipe data
    let nama;       
    nama = "Jokowi";
    println!("{}", nama);

    // variabel tanpa nilai awal dengan tipe data
    let jabatan: &str;
    // jabatan = 23;       // jabatan harus bertipe string slice, bukan integer
    jabatan = "presiden";

    // variabel dengan nilai awal tanpa tipe data
    let slogan = "Kerja kerja kerja";

    // variabel dengan nilai awal dan tipe data
    let slogan: &str = "Kerja kerja kerja";
}

pub fn mutable_variable() {
    let nama = "Jokowi";
    // nama = "Susilo Bambang";    // variabel nama immutable, tidak bisa di assign

    let mut slogan = "Kerja kerja kerja";
    slogan = "Work smart not hard!"
}


pub fn reference() {
    let nama1 = Nama("Jokowi");

    let nama2 = nama1;

    println!("{:p}", nama2.0);
    // println!("{}", nama1.0);
}

pub fn borrowing() {
    let nama1 = Nama("Jokowi");

    let nama2 = &nama1;     // borrowing dengan &

    println!("{}", nama2.0);
    println!("{}", nama1.0);
}


pub fn mutable_borrow() {
    let mut nama1 = Nama("Jokowi");

    let nama2 = &mut nama1;     // mutable borrow dengan &mut

    nama2.0 = "Susilo Bambang";

    println!("{}", nama1.0);
}

pub fn invalid_reference() {
    let nama1 = Nama("Jokowi");

    let nama2 = &nama1;     // mutable borrow dengan &mut
    println!("{}", nama2.0);

    {
        let x = nama1;
        println!("{}", x.0);
    }
}

pub struct Nama<'a>(&'a str);

pub fn multiple_borrow() {
    let nama1 = "Jokowi";

    let nama2 = &nama1;
    let nama3 = &nama1;
    println!("{} {}", nama2, nama3);


    let mut slogan1 = "Kerja kerja kerja";
    let slogan2 = &mut slogan1;
    // let slogan3 = &mut slogan1;      // rust borrow checker tidak mengijinkan kode ini

    *slogan2 = "Kerja smart not hard";
    println!("{}", slogan1);
}
