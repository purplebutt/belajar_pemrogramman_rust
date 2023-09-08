use std::rc::Rc;

type MyType = Option<Rc<Box<String>>>;

pub fn sample_function(number: i32) -> MyType {
    let mut s = String::from("Number: ");
    s = s + &number.to_string();
    Some(Rc::new(Box::new(s)))
}


type F = dyn Fn(i32) -> i32;

pub fn add_one(n: i32) -> i32 {
    n + 1
}

pub fn exec_other_func(n: i32, f: &F) -> i32 {
    f(n)
}


pub fn func1() -> i32 {
    return 23;          // keluar dari fungsi dengan return value 23
    println!("Hi");     // kode ini tidak akan di eksekusi
}

pub fn func2() -> i32 {
    23                  // return value 23
}