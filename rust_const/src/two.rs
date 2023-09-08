// struct Animal {
//     name: String,
//     age: u32
// }
// impl Animal {
//     const fn new(name: &str, age: u32) -> Self {
//         Self { name: String::from(name), age }
//     }
// }


struct Human<'a> {
    name: &'a str,
    age: u32,
}
impl<'a> Human<'a> {
    const fn new(name: &'a str, age: u32) -> Self {
        Self { name, age }
    }
}

const HUME: Human = Human::new("Lyla", 30);

#[allow(non_snake_case)]
pub fn demoTwo() {
    println!("{} {}", HUME.name, HUME.age);
}
