# pdf-jlib

This project is just an experiment to learn about the inner workings of PDF's.
In the meantime I am learning about Rust, because as you can see, I'm not really good at it yet.

## Goals:

- [x] Create a basic PDF file
- [x] Create a page
- [ ] Create multiple pages
- [x] Add some basic text
- [ ] Add images
- [ ] Use encoding algorithms (like zlib/FlateEncode)
- [ ] Change font for text
- [ ] Add embedded fonts

### Ultimate goal

My ultimate goal is to be able to parse a basic existing PDF file and be able to modify it.
No fancy stuff, just the basics.
For example: adding a page, adding text, adding images, adding elements on top of existing pages, etc.

Of course, this also means there will be an API to create PDF files from scratch and adding elements to it. I want to add functionality to work based on milimeters instead of points, because I think it's easier to work with.
