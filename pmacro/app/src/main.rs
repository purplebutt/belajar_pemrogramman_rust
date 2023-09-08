use mylib::Speak;
use mymacro::Speak;
use mymacro::author;
use mymacro::exclude;

#[allow(unused)]
fn main2() {
    author!();
    
    let a = get_author();
    println!("{a}");
}


#[allow(dead_code)]
#[derive(Speak)]
struct Dog {
    name: String,
    sound: String,
    age: u32,
}

#[allow(dead_code)]
#[derive(Speak)]
struct Cat {
    name: String,
    sound: String,
    color: String
}

#[exclude]
fn old_code() {
    println!("Lorem ipsum");
}


#[allow(unused)]
fn main() {
    let d = Dog { 
        name: "Hiro".into(), 
        sound: "Woof..".into(),
        age: 4
    };
    let c = Cat { 
        name: "Miko".into(), 
        sound: "Meow..".into(),
        color: "White".into() 
    };

    d.speak("Woof..");
    d.show_fields();

    c.speak("Meow..");
    c.show_fields();
}

