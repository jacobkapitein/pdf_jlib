use crate::pdf::pdf;
use std::fs::File;
use std::io::{self, Write};

struct XrefTableEntry {
    offset: usize,
    generation: u32,
    free: bool,
}

impl XrefTableEntry {
    fn encode(&self) -> String {
        let offset_initial_string = self.offset.to_string();
        let padding_offset_zeroes = 10 - offset_initial_string.len();
        let offset_string = format!(
            "{}{}",
            "0".repeat(padding_offset_zeroes),
            offset_initial_string
        )
        .to_string();

        let generation_initial_string = self.generation.to_string();
        let padding_generation_zeroes = 5 - generation_initial_string.len();
        let generation_string = format!(
            "{}{}",
            "0".repeat(padding_generation_zeroes),
            generation_initial_string
        );

        let free_string = match self.free {
            true => "f",
            false => "n",
        };

        format!("{} {} {}\n", offset_string, generation_string, free_string).to_string()
    }
}

pub struct Writer {}

impl Writer {
    pub fn write_pdf_to_file(filename: String, mut pdf: pdf::Pdf) -> pdf::Pdf {
        let mut file = File::create(filename + ".pdf").expect("file created");
        let mut object_pointer: usize = 0;
        let mut latest_object_id: u32 = 1;
        let mut xref_table: Vec<XrefTableEntry> = vec![XrefTableEntry {
            offset: object_pointer,
            generation: 65535,
            free: true,
        }];

        object_pointer += Self::write_file_header(&mut file, pdf.pdf_version);
        let catalog_result = Self::write_catalog(&mut file, &latest_object_id);
        xref_table.push(XrefTableEntry {
            offset: object_pointer,
            generation: 0,
            free: false,
        });
        object_pointer += catalog_result;
        latest_object_id += 1;

        let pages_id = latest_object_id.clone();
        let pages_result = pdf.encode_pages(latest_object_id);
        file.write_all(pages_result.0.as_bytes())
            .expect("write pages objects");
        xref_table.push(XrefTableEntry {
            offset: object_pointer,
            generation: 0,
            free: false,
        });
        object_pointer += pages_result.0.len();
        latest_object_id = pages_result.1;

        pdf.pages.iter().for_each(|page| {
            let page_result = page.encode(&pages_id);
            file.write_all(page_result.as_bytes())
                .expect("writing page with id");
            xref_table.push(XrefTableEntry {
                offset: object_pointer,
                generation: 0,
                free: false,
            });
            object_pointer += page_result.len();
        });

        let xref_table_start = object_pointer.clone();
        let mut xref_table_string = String::from("xref\n");
        xref_table_string.push_str(&*format!("0 {}\n", latest_object_id));
        xref_table.iter().for_each(|item| {
            xref_table_string.push_str(&*item.encode());
        });

        file.write_all(xref_table_string.as_bytes())
            .expect("write xref table");

        Self::write_trailer(&mut file, latest_object_id, xref_table_start);

        pdf
    }

    fn write_trailer(output: &mut File, amount_of_objects: u32, xref_table_start: usize) {
        let mut trailer_string = String::from(
            "trailer\n\
            <<\n",
        );
        trailer_string.push_str(&*format!(
            "\x20\x20/Size {}\n\
            \x20\x20/Root 1 0 R\n\
            >>\n\
            startxref\n\
            {}\n\
            %%EOF\n",
            amount_of_objects, xref_table_start
        ));

        output
            .write_all(trailer_string.as_bytes())
            .expect("write file trailer");
    }

    fn write_file_header(output: &mut File, version: &str) -> usize {
        // The seemingly random bytes indicate to PDF readers that this is a binary PDF.
        // Even when we don't use binary data, we could still add it no problem.
        let header: Vec<u8> = [
            b"%PDF-".to_vec(),
            version.as_bytes().to_vec(),
            b"\n%\xC4\xE3\xFB\xA4\n\n".to_vec(),
        ]
        .concat();
        output.write_all(&header).expect("write file header");
        header.len()
    }

    fn write_catalog(output: &mut File, mut latest_object_id: &u32) -> usize {
        let catalog = format!(
            "{} 0 obj\n\
            <<\n\
            \x20\x20/Type /Catalog\n\
            \x20\x20/Pages {} 0 R\n\
            >>\n\
            endobj\n\n",
            latest_object_id,
            latest_object_id + 1
        );
        output
            .write_all(catalog.as_bytes())
            .expect("write file catalog");

        catalog.len()
    }
}
