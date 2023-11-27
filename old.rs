mod pdf;

use pdf::pdf::Pdf;
use std::fs::File;
use std::io::{self, Write};



fn write_file_header(output: &mut File) -> io::Result<()> {
    output.write_all(b"%PDF-1.7\n%\xC4\xE3\xFB\xA4\n\n")?;
    Ok(())
}

struct PdfCatalog {
    id: u32,
    pages: PdfPages,
}

struct PdfPages {
    id: u32,
    count: u32,
    media_box: PdfMediaBox,
}

struct PdfResources {
    font: PdfFont,
}

struct PdfFont {
    id: u32,
    subtype: String,
    base_font: String,
}

struct PdfPage {
    id: u32,
    parent_id: u32,
    media_box: PdfMediaBox,
    contents: PdfContents,
    resources: PdfResources,
}

struct PdfMediaBox {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

struct PdfContents {
    id: u32,
    length: u32,
}

struct PdfXrefEntry {
    offset: u32,
    generation: u32,
    in_use: bool,
}

fn main() -> io::Result<()> {
    // Create a new PDF file
    let mut output = File::create("hello_world.pdf")?;
    let mut pointer: u32 = 16;

    // Page size in points (1 point = 1/72 inch)
    let page_width = 400.0;
    let page_height = 400.0;

    // Text to be added
    let text = "Danielle ik hou van jou <3";

    // Write the PDF header
    write_file_header(&mut output)?;

    // Initialize the PDF
    let catalog = PdfCatalog {
        id: 1,
        pages: PdfPages {
            id: 2,
            count: 1,
            media_box: PdfMediaBox {
                x0: 0.0,
                y0: 0.0,
                x1: page_width,
                y1: page_height,
            },
        },
    };

    // Create a new page
    let page = PdfPage {
        id: 4,
        parent_id: catalog.pages.id,
        media_box: PdfMediaBox {
            x0: 0.0,
            y0: 0.0,
            x1: page_width,
            y1: page_height,
        },
        contents: PdfContents { id: 5, length: 0 },
        resources: PdfResources {
            font: PdfFont {
                id: 3,
                subtype: "Type1".to_string(),
                base_font: "Helvetica-Bold".to_string(),
            },
        },
    };

    // Initialize the xref table
    let mut xref_dict: Vec<PdfXrefEntry> = Vec::new();
    xref_dict.push(PdfXrefEntry {
        offset: 0,
        generation: 65535,
        in_use: false,
    });

    // Write the catalog
    let write_catalog = format!(
        "{} 0 obj % entry point\n\
         <<\n\
         \t/Type /Catalog\n\
         \t/Pages {} 0 R\n\
         >>\n\
         endobj\n\n",
        catalog.id, catalog.pages.id
    );
    output.write_all(write_catalog.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_catalog.len() as u32;

    // Write the pages
    let write_pages = format!(
        "{} 0 obj % parent of pages\n\
         <<\n\
         \t/Type /Pages\n\
         \t/Count {}\n\
         \t/Kids [{} 0 R]\n\
         \t/MediaBox [{} {} {} {}]\n\
         >>\n\
         endobj\n\n",
        catalog.pages.id,
        catalog.pages.count,
        page.id,
        catalog.pages.media_box.x0,
        catalog.pages.media_box.y0,
        catalog.pages.media_box.x1,
        catalog.pages.media_box.y1
    );
    output.write_all(write_pages.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_pages.len() as u32;

    // Write the font
    let write_font = format!(
        "{} 0 obj % font definition\n\
         <<\n\
         \t/Type /Font\n\
         \t/Subtype /{}\n\
         \t/BaseFont /{}\n\
         >>\n\
         endobj\n\n",
        page.resources.font.id, page.resources.font.subtype, page.resources.font.base_font
    );
    output.write_all(write_font.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_font.len() as u32;

    // Write the page
    let write_page = format!(
        "{} 0 obj % first page\n\
         <<\n\
         \t/Type /Page\n\
         \t/Parent {} 0 R\n\
         \t/MediaBox [{} {} {} {}]\n\
         \t/Contents {} 0 R\n\
         \t/Resources <<\n\
         \t\t/Font <<\n\
         \t\t\t/F1 {} 0 R\n\
         \t\t>>\n\
         \t>>\n\
         >>\n\
         endobj\n\n",
        page.id,
        page.parent_id,
        page.media_box.x0,
        page.media_box.y0,
        page.media_box.x1,
        page.media_box.y1,
        page.contents.id,
        page.resources.font.id
    );
    output.write_all(write_page.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_page.len() as u32;

    // Write the contents
    let write_contents = format!(
        "{} 0 obj % content of the first page\n\
         <<\n\
         \t/Length {}\n\
         >>\n\
         stream\n\
         BT\n\
         80 220 TD % x,y coords of text\n\
         /F1 24 Tf\n\
         0 0 Td\n\
         ({}) Tj\n\
         ET\n\
         endstream\n\
         endobj\n\n",
        page.contents.id, page.contents.length, text
    );
    output.write_all(write_contents.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_contents.len() as u32;

    // Write the producer tag
    let pdf_info = format!(
        "10 0 obj\n\
        <<\n\
        \t/Type /Info\n\
        \t/Producer (pdf-jlib 0.0.1)\n\
        \t/Creator (pdf-jlib 0.0.1)\n\
        >>\n\
        endobj\n\n"
    );
    output.write_all(pdf_info.as_bytes())?;
    xref_dict.push(PdfXrefEntry {
        offset: pointer,
        generation: 0,
        in_use: true,
    });
    pointer += write_contents.len() as u32;

    // Write the xref table
    let start_xref = pointer;
    let xref_dict_len = xref_dict.len();
    let mut write_xref = format!("xref % xref table\n0 {}\n", xref_dict_len);
    for entry in xref_dict {
        write_xref.push_str(&format!(
            "{:010} {:05} {}\n",
            entry.offset,
            entry.generation,
            if entry.in_use { "n" } else { "f" }
        ));
    }
    write_xref.push_str("trailer\n");
    write_xref.push_str(&format!(
        "<<\n\
         \t/Size {} % amount of objects in document\n\
         \t/Root {} 0 R % root catalog element reference\n\
         \t/Info 10 0 R\n\
         >>\n\
         startxref % references the position of the xref table\n\
         {}\n\
         %%EOF\n",
        xref_dict_len, catalog.id, start_xref
    ));
    output.write_all(write_xref.as_bytes())?;

    Ok(())
}
