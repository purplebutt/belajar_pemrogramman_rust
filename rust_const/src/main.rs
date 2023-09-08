mod one;
mod two;
mod three;

#[allow(unused_imports)]
use one::demoOne;
#[allow(unused_imports)]
use two::demoTwo;
use three::{withoutConst, withConst};

fn main() {
    demoOne();
    demoTwo();
    println!("{}", "=".repeat(30));
    withoutConst();
    println!("{}", "=".repeat(30));
    withConst();
}

