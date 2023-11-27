use crate::pdf::pdf;
use std::fs::File;
use std::io::{self, Write};

pub struct Writer {}

impl Writer {
    pub fn write_pdf_to_file(filename: String, pdf: &pdf::Pdf) {
        let mut file = File::create(filename + ".pdf").expect("file created");
        let mut object_pointer: u32 = 16;
        let mut latest_object_id: u32 = 1;

        Self::write_file_header(&mut file, pdf.pdf_version);
    }

    fn write_file_header(output: &mut File, version: &str) {
        let header: Vec<u8> = [
            b"%PDF-".to_vec(),
            version.as_bytes().to_vec(),
            b"\n%\xC4\xE3\xFB\xA4\n\n".to_vec(),
        ]
        .concat();
        output.write_all(&header).expect("write file header");
    }
}
