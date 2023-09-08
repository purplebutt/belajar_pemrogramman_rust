pub trait Printable {
    fn print(&self);
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Account {
    name: String,
    password: String,
}


impl Account {
    pub fn new(name: &str, password: &str) -> Self {
        Self { 
            name: name.to_string(), 
            password: password.to_string()
        }
    }
}

impl Printable for Account {
    fn print(&self) {
        println!("{}", self.name);
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Account (name: {})", self.name)
    }
}


pub fn show<T: Printable + std::fmt::Display> (arg: T) {
    arg.print();
    println!("{}", arg);  // error
}

pub fn show2 (arg: impl Printable) {
    arg.print();
}


pub fn call_show() {
    let acc = Account::new("Joko", "okoJ");
    //show2(acc);          // kode ini ok
    //show("Hello");      // kode ini error, tipe "&str" tidak memiliki impl trait Printable
}


impl Printable for &str {
    fn print(&self) {
        println!("{}", self)
    }
}
