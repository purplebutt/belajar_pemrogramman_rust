pub fn di_hukum() {
    let mut i = 1;
    loop {
        if i > 100 {
            return;
        }
        println!("{}\tSaya tidak akan mengulangi", i);
        i += 1;
    }
}

pub fn di_hukum_lagi() {
    let mut i = 1;
    while i <= 100 {
        println!("{}\tSaya tidak akan mengulangi", i);
        i += 1;
    }
}


#[allow(dead_code)]
enum Status {
    Marah(i32),
    Happy
}

pub fn while_let() {
    let ibu_guru = Status::Marah(100);
    let mut i = 1;

    while let Status::Marah(n) = ibu_guru {
        if i > n { break; }
        println!("{}\tSaya tidak akan mengulangi", i);
        i += 1;
    }
}
