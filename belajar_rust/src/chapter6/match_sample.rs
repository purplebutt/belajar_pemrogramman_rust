pub fn match_demo() {
    let my_age = 32;
    let her_age = 35;

    match my_age.cmp(&her_age) {
        std::cmp::Ordering::Equal => println!("Kita sebaya"),
        std::cmp::Ordering::Greater => println!("She call me big bro"),
        std::cmp::Ordering::Less => println!("Dating old-lady")
    }
}