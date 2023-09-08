trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

macro_rules! impl_speak {
    ($target:ident, $sound:literal) => {
        impl Speak for $target {
            fn speak(&self) {
                println!("{} says: {}", stringify!($target), $sound);
            }
        }
    };
}

pub fn demo() {
    let tom = Dog;
    impl_speak!(Dog, "Woof..");
    tom.speak();

    let jerry = Cat;
    impl_speak!(Cat, "Meow..");
    jerry.speak();
}

