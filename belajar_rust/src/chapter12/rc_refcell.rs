use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
pub fn demo() {
    let s0 = Rc::new(
        RefCell::new(
            "Hi".to_string()
        )
    );

    let s1 = s0.clone();        // memanggil clone() dari var s0
    let s2 = Rc::clone(&s0);    // memanggil clone() dari struct Rc

    s1.borrow_mut().push_str(" my");
    s2.borrow_mut().push_str(" Friends 🥰");

    // let mut mutref1 = s1.borrow_mut();
    // let mut mutref2 = s2.borrow_mut();
    // mutref2.push_str(" my");
    // mutref1.push_str(" Friends 🥰");

    println!("{}", s0.borrow());
}


type NBox = RefCell<Rc<Node>>;

#[derive(Debug)]
enum Node {
    Item(String, NBox),
    Empty
}
impl Node {
    fn as_rcbox(self) -> Rc::<Self> {
        Rc::new(self)
    }
    fn empty() -> NBox {
        RefCell::new(
            Self::Empty.as_rcbox()
        )
    }
    fn new(value: &str) -> Self {
        let t = Item("".to_string(), Self::empty());
        Self::Item(value.to_string(), RefCell::new(t.as_rcbox()))
    }
    fn next(&self) -> Option<&NBox> {
        match self {
            Self::Item(_, n) => Some(n),
            Self::Empty => None
        }
    }
}

use self::Node::Item;

pub fn test() {
    let a = Node::new("Satu").as_rcbox();
    println!("a rc count (awal) = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());
    
    let b = Item("Lima".to_string(), RefCell::new(Rc::clone(&a))).as_rcbox();
    println!("a rc count (setelah b) = {}", Rc::strong_count(&a));
    println!("b rc count (awal) = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.next());

    if let Some(n) = a.next() {
        *n.borrow_mut() = b.clone();
        // atau bisa juga dengan kode di bawah ini
        // *n.borrow_mut() = b.clone();
    }

    println!("b rc count (setelah modifikasi a) = {}", Rc::strong_count(&b));
    println!("a rc count (setelah modifikasi a) = {}", Rc::strong_count(&a));

    // code berikut akan mengakibatkan stackoverflow
    // println!("{a:?}");
}

