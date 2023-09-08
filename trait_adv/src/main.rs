mod trait_obj;

#[allow(dead_code)]
mod ambiguity {
    pub trait Human {
        fn run(&self);
    }
    pub trait Animal {
        fn run(&self);
    }
    pub struct SpiderMan {
        name: String
    }
    impl SpiderMan {
        pub fn new(name: &str) -> Self {
            Self { name: name.to_string() }
        }
        // pub fn run(&self) {
        //     println!("SpiderMan {} is running!", self.name)
        // }
    }
    impl Human for SpiderMan {
        fn run(&self) {
            println!("Human ({}) is running!", self.name)
        }
    }
    impl Animal for SpiderMan {
        fn run(&self) {
            println!("Animal ({}) is running!", self.name)
        }
    }
}



mod supertrait {
    pub trait Horse {
        fn run(&self) -> String;
    }
    pub trait Fly {
        fn fly(&self) -> String;
    }
    pub trait FlyHorse: Horse + Fly {
        // default implementation
        fn runfly(&self) {
            let r = self.run();
            let f = self.fly();
            println!("{r} and {f}")
        }
    }
    pub struct Pegasus;
    impl Horse for Pegasus {
        fn run(&self) -> String {
            format!("Pegasus is running!")
        } 
    }
    impl Fly for Pegasus {
        fn fly(&self) -> String {
            format!("Pegasus is flying!")
        } 
    }
    impl FlyHorse for Pegasus {}
}



fn static_dispatch<FH: supertrait::FlyHorse>(p: &FH) {
    p.runfly()
}
fn dynamic_dispatch(p: &dyn supertrait::FlyHorse) {
    p.runfly()
}

trait Convert<To> {
    fn convert(&self) -> To;
}

struct Prima(u32);
struct Ganjil(u32);

impl Convert<Ganjil> for Prima {
    fn convert(&self) -> Ganjil {
        if self.0 % 2 != 0 {
            Ganjil(self.0)
        } 
        else {
            panic!("{} is not an Odd number", self.0);
        }
    }  
}
impl Convert<Prima> for Ganjil {
    fn convert(&self) -> Prima {
        let mut is_prime = true;
        let mut n = self.0;
        while n > 2 {
            n -= 1;
            if self.0 % n == 0 {
                is_prime = false;
                break
            }
        }
        if is_prime {
            Prima(self.0)
        } 
        else {
            panic!("{} is not a Prime number", self.0);
        }
    }  
}

// fn show<T: Convert<Ganjil>>(number: T) {
fn show(number: impl Convert<Ganjil>) {
    let value = number.convert();
    println!("{}", value.0);
}

fn print(number: &dyn Convert<Ganjil>) {
    let value = number.convert();
    println!("{}", value.0);
}

// fn show(number: Prima) {
//     let value = number.convert();
//     println!("{}", value.0);
// }

fn main() {
    use crate::trait_obj::test;

    test();
}

fn main1() {
    let lima = Prima(5);
    print(&lima);
    show(lima);

    let x = file!();
    println!("{x}");

    let l = line!();
    println!("{l}");

    let mp = module_path!();
    println!("{mp}");
    

    // let dua = Prima(2);
    // let ganjil2 = dua.convert();
    // println!("{}", ganjil2.0);

    // let tujuh = Ganjil(9);
    // let prime = tujuh.convert();
    // println!("{}", prime.0);
}


fn main2() {
    use supertrait::*;

    let pegasus = Pegasus;
    static_dispatch(&pegasus);
    dynamic_dispatch(&pegasus);
}

fn main3() {
    use ambiguity::*;
    let peter = SpiderMan::new("Peter");

    // peter.run();
    // SpiderMan::run(&peter);
    Animal::run(&peter);
    Human::run(&peter);
    <SpiderMan as Animal>::run(&peter);
    <SpiderMan as Human>::run(&peter);
}

