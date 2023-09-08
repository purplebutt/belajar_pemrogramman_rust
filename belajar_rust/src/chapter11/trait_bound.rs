pub fn accept_all<A, B> (a: A, b: B) {
    println!("Hello");
}

pub fn call_accept_all() {
    accept_all(12, "Hello");
    accept_all(Vec::<u32>::new(), String::new());
    accept_all(||{ println!("Hi")}, Box::new(23.5));
    accept_all((1,"dua"), [0u8; 125]);
}
