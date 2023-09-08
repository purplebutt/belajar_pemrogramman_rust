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
    pub fn build() -> ClrPrintBuilder<'a> {
        ClrPrintBuilder::new(
            ClrPrint::new("", "", "", "")
        )
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

pub struct ClrPrintBuilder<'a> {
    item: ClrPrint<'a>
}

impl<'a> ClrPrintBuilder<'a> {
    fn new(item: ClrPrint<'a>) -> Self {
        Self { item }
    }
    fn build() -> Self {
        Self { 
            item: ClrPrint::new("", "", "", "")
        }
    }
    pub fn set_text(mut self, text: &'a str) -> Self {
        self.item.text = text; self
    }
    pub fn set_opening(mut self, opening: &'a str) -> Self {
        assert_eq!(1, opening.len());
        self.item.opening = opening; self
    }
    pub fn set_closing(mut self, closing: &'a str) -> Self {
        assert_eq!(1, closing.len());
        self.item.closing = closing; self
    }
    pub fn set_color(mut self, color: &'a str) -> Self {
        self.item.color = color; self
    }
    pub fn finish(self) -> ClrPrint<'a> {
        self.item
    }
}

pub const CLR: &str = "\x1B[0m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const BLUE: &str = "\x1B[34m";


pub fn with_builder() {
    let clrprint = ClrPrint::build()
        .set_text("Hello")
        .set_color(RED)
        .set_opening("#")
        .set_closing("!")
        .finish();

    clrprint.print();
}

