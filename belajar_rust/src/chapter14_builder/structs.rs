use std::marker::PhantomData;
use super::traits::{Builder, Buildable, SkipTo, Finishable};


// definsi struct ClrPrint
pub struct ClrPrint {
    text: String
}

// implementasi struct ClrPrint
impl ClrPrint {
    // pub fn text(&self) -> &str { &self.text }
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
// impl Buildable<ClrPrintBuilder<Start>> for ClrPrint { }

#[allow(private_in_public)]
trait Stage { }     // marker trait untuk stage

// Unit struct untuk stage 
pub struct Start{}  // stage awal atau stage no.1
pub struct Two{} 
pub struct Three{} 
pub struct Four{} 
pub struct Five {}
pub struct End{}

impl Stage for Start { } 
impl Stage for Two { } 
impl Stage for Three { } 
impl Stage for Four { } 
impl Stage for Five { } 
impl Stage for End { }


// definsi struct ClrPrintBuilder
pub struct ClrPrintBuilder<Stage> {
    item: ClrPrint,
    marker: PhantomData<Stage>
}

// implementasi untuk struct ClrPrintBuilder
impl Builder<ClrPrint> for ClrPrintBuilder<Start> { }
// impl Builder<ClrPrint> for ClrPrintBuilder<Start> { 
//     fn default() -> Self {
//         Self { 
//             item: ClrPrint { text: "".into()},
//             marker: PhantomData::default()
//         }
//     }
// }

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
        const BOLD: &str = "\x1B[1m";
        if bold {
            self.item.text.push_str(BOLD);
        }
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Three> {
    pub fn build_underline(mut self, underline: bool) -> ClrPrintBuilder<Four> {
        const UNDERLINE: &str = "\x1B[4m";
        if underline {
            self.item.text.push_str(UNDERLINE);
        }
        ClrPrintBuilder::new(self.item)
    }
}
impl ClrPrintBuilder<Four> {
    pub fn build_blink(mut self, blink: bool) -> ClrPrintBuilder<Five> {
        const BLINK: &str = "\x1B[5m";
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

