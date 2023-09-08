use std::rc::Rc;


type MyType = Option<Rc<Box<String>>>;

pub fn type_sample() {
    
    // let data: Option<Rc<Box<String>>>;
    let data: MyType;

    let b = Box::new(String::from("Hello"));
    data = Some(Rc::new(b));

    println!("{}", data.unwrap());
}