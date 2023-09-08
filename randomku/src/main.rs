use randomku::Rng;


fn main() {
    let mut rng = Rng::new();

    for _i in 1..=15 {
        let random_number = rng.rnd_between(1.0, 10.0);
        print!("{}, ", random_number);
    }
    println!();
    println!("{}", "-".repeat(40));
    for i in 1..=10 {
        let random_number = rng.rnd_between_f64(1.0, 10.0);
        println!("{} => {}", i, random_number);
    }
}
