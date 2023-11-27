use crate::pdf;
use crate::pdf::writer::Writer;

pub struct Pdf {
    producer: String,
    creator: String,
    pub pdf_version: &'static str,
    pages: Vec<pdf::page::Page>,
}

impl Pdf {
    pub fn new() -> Self {
        Self {
            producer: String::from("pdf-jlib 0.0.1"),
            creator: String::from("pdf-jlib 0.0.1"),
            pdf_version: "1.7",
            pages: Vec::new(),
        }
    }

    pub fn set_producer(&mut self, producer: String) {
        self.producer = String::from(producer);
    }

    pub fn set_creator(&mut self, creator: String) {
        self.creator = creator;
    }

    pub fn add_page(&mut self, size: (u32, u32)) -> &pdf::page::Page {
        let page = pdf::page::Page::new(size);
        self.pages.push(page);

        &self.pages[self.pages.len() - 1]
    }

    pub fn write_to_file(&self, filename: String) {
        Writer::write_pdf_to_file(filename, self);
    }
}
