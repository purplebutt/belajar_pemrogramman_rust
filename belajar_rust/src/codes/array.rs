pub fn array_demo() {
    let hari: [&str; 7] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at", "Sabtu", "Minggu"];

    let _numbera = [0; 5];
    let _numberb = [0, 0, 0, 0, 0];

    let hari2 = hari[1];    // zero-base index. Index di mulai dari 0

    println!("{}", hari2)         // will print "Selasa"
}


pub fn array_demo2() {
    let presiden = ["Soekarno", "Soeharto", "Habibie", "Gus Dur", "Megawati", "Sby", "Jokowi"];

    let presiden8 = presiden[8];    // out of bounds index. Tidak bisa di kompile

    println!("{}", presiden8);
}