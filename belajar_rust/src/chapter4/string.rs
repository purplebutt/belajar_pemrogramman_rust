
pub fn string_demo() {
    let byte_array: [u8; 11] = [
        0b01000001,
        0b01110000,
        0b01100001,
        0b00100000,
        0b01101011,
        0b01100001,
        0b01100010,
        0b01100001,
        0b01110010,
        0b00100000,
        0b00111111
    ];

    for i in byte_array.into_iter() {
        print!("{}", i as char)
    }
}

pub fn string_literal1() {
    let mut presiden: &str = "Jokowi";
    println!("address: {:p}; value: {}", presiden, presiden);       // output "Jokowi"

    presiden = "Joko Widodo";
    println!("address: {:p}; value: {}", presiden, presiden);       // output "Jokowi Widodo"
}

pub fn string_slice() {
    let presiden: &str = "Jokowi Widodo";

    let nama = &presiden[0..4];
    assert_eq!("Joko", nama);

    let marga = &presiden[7..];
    assert_eq!("Widodo", marga);

    let nickn = &presiden[2..6];
    assert_eq!("kowi", nickn);
}

pub fn string_demo2() {
    let mut slogan = String::new();
    
    slogan.push_str("Work");
    slogan.push(' ');
    slogan.push_str("work ");
    slogan.push_str("work ");

    println!("{}", slogan);      // output "Work work work"

    let _upper = slogan.to_uppercase();     // _upper = "WORK WORK WORK"
    let _lower = slogan.to_lowercase();     // _lower = "work work work"
    let _contain_ker = slogan.contains("wor");      // _contain_ker = true
    let mut replace = slogan.replace(" work", ""); // replace = "Work "
    replace.insert_str(replace.len(), "smart not hard");
    println!("{}", replace);    // output "Work smart not hard"
}

pub fn string_demo3() {
    use std::mem;

    let s = String::from("Pancasila");
    let ss = "Pancasila";

    println!("String: {:?} bytes, string slice: {:?} bytes", mem::size_of_val(&s), mem::size_of_val(ss));
}

pub fn string_concat() {
    let mut s1 = String::from("Damn");
    let s2 = "I love";
    
    s1 = s1 + " " + s2;

    s1 = format!("{} {}", s1, "Indonesia");

    println!("{}", s1);
}


pub fn string_iterate() {
    let s = "Jok".to_string();

    let mut c = s.chars();
    println!("{}", c.next().unwrap());
    println!("{}", c.next().unwrap());
    println!("{}", c.next().unwrap());


    let mut b = s.bytes();
    println!("{}", b.next().unwrap());
    println!("{}", b.next().unwrap());
    println!("{}", b.next().unwrap());
}