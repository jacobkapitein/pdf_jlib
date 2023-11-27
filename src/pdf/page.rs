use crate::pdf;

pub struct Page {
    size: (u32, u32),
    contents: Vec<pdf::content::Content>,
}

impl Page {
    pub(crate) fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            contents: Vec::new(),
        }
    }

    pub fn add_content(&mut self, text: String, position: (i32, i32)) -> &pdf::content::Content {
        let content = pdf::content::Content::new(text, position);
        self.contents.push(content);

        &self.contents[self.contents.len() - 1]
    }
}
