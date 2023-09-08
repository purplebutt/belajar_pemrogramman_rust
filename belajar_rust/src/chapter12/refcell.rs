use std::cell::RefCell;
use std::rc::Rc;

pub fn int_mut() {
    // let b = Box::new(String::from("Hello"));
    let mut b = Box::new(String::from("Hello"));

    let mut bmut = b.as_mut();

    *bmut = "Apa kabar".into();

    println!("{}", b);
}

//use std::cell::RefCell;
pub fn int_mut_cell() {
    let b = RefCell::new(String::from("Hello"));

    let mut bmut = b.borrow_mut();

    *bmut = "Apa kabar".into();

    println!("{}", bmut);
}

pub fn refcell_demo() {
    let s = String::from("merah");
    let rc = Rc::new(RefCell::new(s));
    
    println!("{}", rc.borrow());

    let mut smut1 = rc.borrow_mut();
    *smut1 = "putih".to_string();

    println!("{}", smut1);
}

pub fn refcell_demo2() {
    let s = String::from("wow");
    let rc = RefCell::new(s);

    let mut smut1 = rc.borrow_mut();
    let mut smut2 = rc.borrow_mut();
    *smut1 = "cool".to_string();
    *smut2 = "cool".to_string();

    println!("{}", smut1);
    println!("{}", smut2);
}

