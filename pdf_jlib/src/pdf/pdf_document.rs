use crate::core::name::Name;
use crate::traits::serialize::Serialize;

use crate::core::dictionary::Dictionary;

pub struct PdfDocument {
    product: String,
    creator: String,
    pub pdf_version: &'static str,
    pub catalog: Dictionary,
    pub objects: Vec<Dictionary>,
}

impl PdfDocument {
    pub fn new() -> Self {
        let mut new_object = Self {
            product: String::from("pdf_jlib 0.0.1"),
            creator: String::from("pdf_jlib 0.0.1"),
            pdf_version: "1.7",
            catalog: Dictionary::new(1),
            objects: Vec::new(),
        };

        new_object
            .catalog
            .add_entry(Name::new("Type"), Box::new(Name::new("Catalog")));

        new_object
    }

    pub fn set_producer(&mut self, producer: &str) {
        self.product = producer.to_string();
    }

    pub fn set_creator(&mut self, creator: &str) {
        self.creator = creator.to_string();
    }

    pub fn add_page(&mut self, page: Dictionary) {
        self.objects.push(page);
    }

    /// The Serialize trait is not used for PdfDocument, because PdfDocument is not a PDF Core Object
    pub fn serialize(&self) -> Vec<u8> {
        let mut xref_table = Vec::new();

        let mut output = Vec::new();

        output.extend_from_slice(b"%PDF-1.7\n\n");

        xref_table.push(output.len());
        output.extend_from_slice(self.catalog.serialize().as_slice());
        xref_table.push(output.len());

        for dict in &self.objects {
            output.extend_from_slice(dict.serialize().as_slice());
            xref_table.push(output.len());
        }
        output.extend_from_slice("\n".as_bytes());

        let startxref = output.len();

        output.extend_from_slice(b"xref\n");
        output.extend_from_slice(b"0 ");
        output.extend_from_slice(xref_table.len().to_string().as_bytes());

        output.extend_from_slice(b"\n");
        output.extend_from_slice("0000000000 65535 f\n".as_bytes());
        for xref in xref_table.iter() {
            let mut xref_str = xref.to_string();

            // Ensure it's not longer than 10 characters, panic if so
            if xref_str.len() > 10 {
                panic!("xref is longer than 10 characters!");
            }

            // Prepend '0' if the length is less than 10
            while xref_str.len() < 10 {
                xref_str.insert(0, '0');
            }

            // Convert to bytes and extend the output
            output.extend_from_slice(xref_str.as_bytes());
            output.extend_from_slice(b" 00000 n\n");
        }

        output.extend_from_slice(b"trailer\n");
        output.extend_from_slice(b"<<\n");
        output.extend_from_slice("/Size ".as_bytes());
        output.extend_from_slice(xref_table.len().to_string().as_bytes());
        output.extend_from_slice(b"\n");
        output.extend_from_slice("/Root 1 0 R\n".as_bytes());
        output.extend_from_slice(">>\n".as_bytes());
        output.extend_from_slice("startxref\n".as_bytes());
        output.extend_from_slice(startxref.to_string().as_bytes());
        output.extend_from_slice(b"\n%%EOF");

        output
    }
}
