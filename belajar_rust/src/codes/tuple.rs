pub fn tuple_sample() {
    let t = ("Yes", 1);

    let yes = t.0;
    let one = t.1;

    println!("{} {}", yes, one);


    // destructuring a tuple
    let list = ("Red", 12u16, 0.7f32, -12i32);

    let (a, b, c, d) = list;

    println!("{} {} {} {}", a, b, c, d)
}