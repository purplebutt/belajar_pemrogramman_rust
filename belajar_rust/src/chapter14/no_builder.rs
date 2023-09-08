pub struct ClrPrint<'a> {
    text: &'a str,
    opening: &'a str,
    closing: &'a str,
    color: &'a str
}

impl<'a> ClrPrint<'a> {
    fn new(text: &'a str, opening: &'a str, closing: &'a str, color: &'a str) -> Self {
        Self { text, opening, closing, color }
    }
    pub fn print(&self) {
        println!("{}{}{}{}{}", 
            self.color, 
            self.opening, 
            self.text, 
            self.closing,
            CLR
        );
    }
}

pub const CLR: &str = "\x1B[0m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const BLUE: &str = "\x1B[34m";


pub fn no_builder() {
    let clrprint = ClrPrint::new(
            "Hello", "<", ">", GREEN
        );
    clrprint.print();
}
