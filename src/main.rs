mod pdf;

use crate::pdf::writer::Writer;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut pdf = pdf::pdf::Pdf::new();
    // This is just some testing..
    for n in 1..100 {
        pdf.add_page((200, 200));
        pdf.add_page((150, 150));
    }

    Writer::write_pdf_to_file(String::from("hello_world_new"), pdf);

    Ok(())
}
