trait Animal {
    fn run(&self);
}

trait Vehicle {
    fn run(&self);
}

struct Something;

impl Animal for Something {
    fn run(&self) {
        println!("an animal is running!");
    }
}

impl Vehicle for Something {
    fn run(&self) {
        println!("a vehicle is running!")
    }
}

fn foo(vehicle: impl Vehicle) {
    vehicle.run()
}
fn bar(vehicle: impl Animal) {
    vehicle.run()
}
pub fn baz() {
    let something = Something;

    bar(something);
}
