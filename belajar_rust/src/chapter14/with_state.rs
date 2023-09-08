use std::marker::PhantomData;

pub trait Builder<T> { }

pub trait SkipTo<To> {
    fn skip(self, text: &str) -> To;
}
pub trait Finishable {
    type Output;
    fn finish(self) -> Self::Output;
}

pub trait Buildable<B: Builder<Self>> where Self: Sized {
    fn builder() -> B;
}

#[derive(Debug)]
pub struct ClrPrint {
    text: String
}

#[allow(dead_code)]
impl ClrPrint {
    pub fn text(&self) -> &str { &self.text }
    pub fn print(&self) {
        const CLR: &str = "\x1B[0m";
        println!("{}{}", self.text, CLR);
    }
}
impl Buildable<ClrPrintBuilder<Start>> for ClrPrint {
    fn builder() -> ClrPrintBuilder<Start> {
        ClrPrintBuilder::default()
    } 
}


pub struct Start{} 
pub struct Two{} 
pub struct Three{} 
pub struct Four{} 
pub struct Five {}
pub struct End{} 
trait Stage { }

impl Stage for Start { } 
impl Stage for Two { } 
impl Stage for Three { } 
impl Stage for Four { } 
impl Stage for Five { } 
impl Stage for End { }

pub struct ClrPrintBuilder<Stage> {
    item: ClrPrint,
    marker: PhantomData<Stage>
}

impl Builder<ClrPrint> for ClrPrintBuilder<Start> { }

impl<S: Stage> ClrPrintBuilder<S> {
    fn new(item: ClrPrint) -> Self {
        Self { 
            item, 
            marker: PhantomData::default() 
        }
    }
}

impl ClrPrintBuilder<Start> {
    fn default() -> Self {
        Self { 
            item: ClrPrint { text: "".into()},
            marker: PhantomData::default()
        }
    } 
    pub fn build_color(mut self, color: &str) -> ClrPrintBuilder<Two> {
        self.item.text = color.into();
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Two> {
    pub fn build_bold(mut self, bold: bool) -> ClrPrintBuilder<Three> {
        if bold {
            self.item.text.push_str(BOLD);
        }
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Three> {
    pub fn build_underline(mut self, underline: bool) -> ClrPrintBuilder<Four> {
        if underline {
            self.item.text.push_str(UNDERLINE);
        }
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Four> {
    pub fn build_blink(mut self, blink: bool) -> ClrPrintBuilder<Five> {
        if blink {
            self.item.text.push_str(BLINK);
        }
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Five> {
    pub fn build_text(mut self, text: &str) -> ClrPrintBuilder<End> {
        self.item.text.push_str(text);
        ClrPrintBuilder::new(self.item)
    }
}

// impl Finishable
impl Finishable for ClrPrintBuilder<End> {
    type Output = ClrPrint;
    fn finish(self) -> Self::Output {
        self.item
    }
}

// impl SkipTo
impl SkipTo<ClrPrintBuilder<End>> for ClrPrintBuilder<Two> {
    fn skip(mut self, text: &str) -> ClrPrintBuilder<End> {
        self.item.text.push_str(text);
        ClrPrintBuilder::new(self.item)
    }
}
impl SkipTo<ClrPrintBuilder<End>> for ClrPrintBuilder<Three> {
    fn skip(mut self, text: &str) -> ClrPrintBuilder<End> {
        self.item.text.push_str(text);
        ClrPrintBuilder::new(self.item)
    }
}

pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const BLUE: &str = "\x1B[34m";
pub const BOLD: &str = "\x1B[1m";
pub const UNDERLINE: &str = "\x1B[4m";
pub const BLINK: &str = "\x1B[5m";

pub fn with_state_demo() {
    let builder = ClrPrint::builder()
        .build_color(RED)
        .build_bold(true)
        .build_underline(true)
        .build_blink(true)
        .build_text("Hallo Semua")
        .finish();

    builder.print();
}

