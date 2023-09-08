use std::sync::Arc;
use std::thread;

fn total(numbers: Vec<i32> ) -> i32 {
    numbers.iter().sum()
}

pub fn demo_closure() {
    let numbers = vec![1,3,5,7,11];
    // let nclone = numbers.clone();

    let handler = thread::spawn(||{
        total(numbers)
    });

    let total = handler.join().unwrap();

    println!("total: {}", total);
    // println!("total of {:?} = {}", nclone, total);
}


fn total_arc(numbers: &Vec<i32> ) -> i32 {
    numbers.iter().sum()
}
pub fn demo_arc() {
    let numbers = Arc::new(vec![1,3,5,7,11]);

    let numbers1 = numbers.clone();
    let handler = thread::spawn(move||{
        total_arc(&numbers1)
    });

    let total = handler.join().unwrap();

    println!("total: {}", total);
    println!("total of {:?} = {}", numbers, total);
}

