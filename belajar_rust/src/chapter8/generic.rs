pub fn print_all_str(data: &[&str]) {
    for i in data.iter() {
        println!("{}", i)
    }
}

pub fn print_all_int(data: &[i32]) {
    for i in data.iter() {
        println!("{}", i)
    }
}

// pub fn print_all_intstr<T: std::fmt::Display>(data: &[T]) {
//     for i in data.iter() {
//         println!("{}", i)
//     }
// }

pub fn print_all_intstr<T>(data: &[T]) {
    for i in data.iter().enumerate() {
        println!("item {}", i.0);
    }
}

pub fn mains() {
    let names = ["Joko", "Fika", "Anies", "Mario"];
    let numbers = [2,5,3,8];

    // print_all_str(&names);
    // print_all_int(&numbers);
    print_all_intstr(&names);
    print_all_intstr(&numbers);
}

pub fn must_print_1() {
    let d = ["Joko", "Fika", "Anies", "Mario"];

    print_all_str(&d);
}

pub fn must_print_2() {
    let d = [2,5,3,8];

    print_all_int(&d);
}
