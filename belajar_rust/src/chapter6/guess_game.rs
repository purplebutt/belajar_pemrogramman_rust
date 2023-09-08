
pub fn gg() {
    let secret = "4";
    let mut guess = String::new();

    println!("Masukkan angka 1 s/d 5:");

    while let Ok(_) =  std::io::stdin().read_line(&mut guess) {
        guess = guess.replace("\n", "");
        if guess == secret {
            println!("Selamat! tebakan anda benar");
            return;
        }
        else {
            println!("Tebakan anda '{}' salah, silahkan coba lagi", guess);
            guess.clear();
        }
    }
}


fn generate_secret(_min: i32, _max: i32) -> i32 {
    76
}

fn process_guess(g: i32, secret: &i32, counter: &i32) -> bool {
    match g.cmp(&secret) {
        std::cmp::Ordering::Equal => {
            print!("Selamat!, tebakan anda benar. "); 
            println!("Anda berhasil menebak dalam {} tebakan", counter); 
            println!("Score anda: {}", 100/counter); 
            return true
        }
        std::cmp::Ordering::Greater => println!("Terlalu besar. Coba lagi dengan angka yang lebih kecil"),
        std::cmp::Ordering::Less => println!("Terlalu kecil. Coba lagi dengan angka yang lebih besar"),
    }
    false
}

pub fn cguess() {
    let minmax = (1, 100);
    let secret: i32 = generate_secret(minmax.0, minmax.1);
    let mut guess = String::new();
    let mut counter = 0;

    println!("Masukkan angka {} s/d {}:", minmax.0, minmax.1);

    while let Ok(_) =  std::io::stdin().read_line(&mut guess) {

        counter += 1;
        if guess == "exit()\n" { println!("Bye"); return; }

        if let Ok(g) = guess.replace("\n", "").parse::<i32>() {
            if process_guess(g, &secret, &counter) { return; }
        }
        else {
            println!("Data harus bertipe angka!")
        }
        guess.clear();      // kosongkan buffer
    }
}