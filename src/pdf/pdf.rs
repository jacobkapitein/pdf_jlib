use crate::pdf;
use crate::pdf::page::Page;
use crate::pdf::writer::Writer;

pub struct Pdf {
    producer: String,
    creator: String,
    pub pdf_version: &'static str,
    pub pages: Vec<pdf::page::Page>,
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

    pub fn write_to_file(self, filename: String) {
        Writer::write_pdf_to_file(filename, self);
    }

    pub(crate) fn encode_pages(&mut self, mut latest_object_id: u32) -> (String, u32) {
        let mut kids_string = String::new();
        let pages_object_id = latest_object_id.clone();
        latest_object_id += 1;
        let pages_length = self.pages.len();

        self.pages
            .iter_mut()
            .enumerate()
            .for_each(|(index, mut page)| {
                match page.id {
                    None => page.set_id(latest_object_id),
                    _ => {}
                }
                kids_string.push_str(&*format!("{} 0 R", page.id.unwrap()));
                if index != pages_length - 1 {
                    kids_string.push_str(" ");
                }
                latest_object_id += 1;
            });

        (
            format!(
                "{} 0 obj\n\
            <<\n\
            \x20\x20/Type /Pages\n\
            \x20\x20/Count {}\n\
            \x20\x20/Kids [{}]\n\
            >>\n\
            endobj\n\n",
                pages_object_id,
                self.pages.len(),
                kids_string
            ),
            latest_object_id,
        )
    }
}
