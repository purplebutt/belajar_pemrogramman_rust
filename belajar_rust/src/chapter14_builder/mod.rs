pub mod traits;
pub mod structs;
pub mod demo;
pub mod entry;


use structs::ClrPrint;
use traits::{Buildable, SkipTo, Finishable};

// konstanta untuk format warna pada terminal
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const BLUE: &str = "\x1B[34m";

pub fn builder_demo() {
    let builder = ClrPrint::builder()
        .build_color(RED)
        .skip("Chomky love to chomp!")
        .finish();

    builder.print();
}

