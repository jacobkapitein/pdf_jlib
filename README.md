# pdf-jlib

This project is just an experiment to learn about the inner workings of PDF's.
In the meantime I am learning about Rust.

## Goals:

Next step: add an API for adding text!
Features I want to add immediately:

- Add text
- Set position
- Set max width
- Rotate

- [x] Create a basic PDF file
- [x] Create a page (manually)
- [ ] Create pages using a higher-level API
- [ ] Add some basic text (default font)
- [ ] Change font for text (default baked in PDF fonts)
- [ ] Add embedded fonts and add to text
- [ ] Add embedded images
- [ ] Use encoding algorithms (like zlib/FlateEncode)

### Ultimate goal

My ultimate goal is to be able to parse a basic existing PDF file and be able to modify it.
No fancy stuff, just the basics.
For example: adding a page, adding text, adding images, adding elements on top of existing pages, etc.

Of course, this also means there will be an API to create PDF files from scratch and adding elements to it. I want to add functionality to work based on milimeters instead of points, because I think it's easier to work with.
