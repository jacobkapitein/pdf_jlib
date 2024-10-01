use pdf_jlib::core::array::Array;
use pdf_jlib::core::dictionary::Dictionary;
use pdf_jlib::core::name::Name;
use pdf_jlib::pdf::pdf_document::PdfDocument;

fn main() {
    let mut pdf = PdfDocument::new();

    pdf.set_creator("Override");
    pdf.set_producer("Override");

    pdf.catalog.add_entry(Name::new("Pages"), Box::new("2 0 R"));

    let mut pages = Dictionary::new(2);
    pages.add_entry(Name::new("Type"), Box::new(Name::new("Pages")));
    pages.add_entry(
        Name::new("MediaBox"),
        Box::new(Array::new(vec![
            Box::new(0),
            Box::new(0),
            Box::new(250),
            Box::new(250),
        ])),
    );
    pages.add_entry(Name::new("Count"), Box::new("2"));
    pages.add_entry(
        Name::new("Kids"),
        Box::new(Array::new(vec![Box::new("3 0 R"), Box::new("4 0 R")])),
    );
    pdf.objects.push(pages);

    let mut page1 = Dictionary::new(3);
    page1.add_entry(Name::new("Type"), Box::new(Name::new("Page")));
    page1.add_entry(Name::new("Parent"), Box::new("2 0 R"));
    pdf.objects.push(page1);

    let mut page2 = Dictionary::new(4);
    page2.add_entry(Name::new("Type"), Box::new(Name::new("Page")));
    page2.add_entry(Name::new("Parent"), Box::new("2 0 R"));
    page2.add_entry(
        Name::new("MediaBox"),
        Box::new(Array::new(vec![
            Box::new(0),
            Box::new(0),
            Box::new(230),
            Box::new(230),
        ])),
    );
    pdf.objects.push(page2);

    let serialized_output = pdf.serialize();

    // Write output to .pdf file
    std::fs::write("output.pdf", serialized_output).expect("Unable to write file");

    println!("Finished!");
}
