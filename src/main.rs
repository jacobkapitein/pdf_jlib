mod pdf;

use crate::pdf::writer::Writer;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut pdf = pdf::pdf::Pdf::new();

    Writer::write_pdf_to_file(String::from("hello_world_new"), &pdf);

    Ok(())
}
