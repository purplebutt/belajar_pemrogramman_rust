// konstanta untuk format font style pada terminal
const BOLD: &str = "\x1B[1m";
const UNDERLINE: &str = "\x1B[4m";
const BLINK: &str = "\x1B[5m";

pub struct ClrPrint {
    text: String
}

impl ClrPrint {
    pub fn new(text: &str, color: &str, is_bold: bool, 
        is_underline: bool, is_blink: bool) -> Self {
        let mut s = "".to_string();
        s.push_str(color);
        if is_bold { s.push_str(BOLD) }
        if is_underline { s.push_str(UNDERLINE) }
        if is_blink { s.push_str(BLINK) }
        s.push_str(text);
        Self { text: s }
    }
    pub fn print(&self) {
        const CLR: &str = "\x1B[0m";
        println!("{}{}", self.text, CLR);
    }

    // example 2
    pub fn default() -> Self {
        let text = "".to_string();
        Self { text }
    }
    pub fn set_text(&mut self, text: &str) {
        self.text.push_str(text);
    }
    pub fn set_color(&mut self, color: &str) {
        self.text.push_str(color);
    }
    pub fn set_bold(&mut self) {
        self.text.push_str(BOLD);
    }
    pub fn set_underline(&mut self) {
        self.text.push_str(UNDERLINE);
    }
    pub fn set_blink(&mut self) {
        self.text.push_str(BLINK);
    }
}


const RED: &str = "\x1B[31m";
const YELLOW: &str = "\x1B[32m";
pub fn demo1() {
    // let cprint = ClrPrint::new("Nusantara", RED, true, true, true);
    let mut cprint = ClrPrint::default();

    cprint.set_color(YELLOW);
    cprint.set_text("Nusantara");
    cprint.print();
}

pub fn this_is_bad() {
    let mut cprint = ClrPrint::default();

    cprint.print();
}

pub fn this_is_bad_too() {
    let mut cprint = ClrPrint::default();

    cprint.set_color(YELLOW);
    cprint.set_bold();

    cprint.print();
}

pub fn this_is_also_bad() {
    let mut cprint = ClrPrint::default();

    cprint.set_text("Nusantara");
    cprint.set_color(YELLOW);

    cprint.print();
}
