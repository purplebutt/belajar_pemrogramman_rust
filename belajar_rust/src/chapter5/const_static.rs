
const PI: f64 = 3.14;
static mut FLAG: bool = false;

pub fn const_static_sample() {
    unsafe {
        println!("flag: {}", FLAG);
        FLAG = true;
        println!("flag: {}", FLAG);
    }

    println!("{}", PI);
}
