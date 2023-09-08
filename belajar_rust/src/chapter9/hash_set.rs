use std::collections::HashSet;
use std::collections::VecDeque;


pub fn hashset_demo() {
    let mut color_set = HashSet::new();

    color_set.insert("Red");
    color_set.insert("Green");
    color_set.insert("Red");
    color_set.insert("Blue");

    for color in color_set.iter() {
        println!("{}", color)
    }

    let x = color_set.get("Red").unwrap();
    println!("==={}===", x);
}

pub fn vecdeque_demo() {
    let mut numbers = VecDeque::<i32>::new();

    numbers.push_front(1);
    numbers.push_front(2);

    for (index, value) in numbers.iter().enumerate() {
        println!("{}\t{}", index, value);
    }

    println!("{}", "#".repeat(30));

    numbers.push_back(0);
    numbers.push_back(1);
    numbers.push_front(3);

    for (index, value) in numbers.iter().enumerate() {
        println!("{}\t{}", index, value);
    }
}