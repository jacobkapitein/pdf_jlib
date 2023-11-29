use crate::pdf;

pub struct Page {
    pub id: Option<u32>,
    size: (u32, u32),
    contents: Vec<pdf::content::Content>,
}

impl Page {
    pub(crate) fn new(size: (u32, u32)) -> Self {
        Self {
            id: None,
            size,
            contents: Vec::new(),
        }
    }

    pub fn add_content(&mut self, text: String, position: (i32, i32)) -> &pdf::content::Content {
        let content = pdf::content::Content::new(text, position);
        self.contents.push(content);

        &self.contents[self.contents.len() - 1]
    }

    pub(crate) fn set_id(&mut self, id: u32) {
        self.id = Option::from(id);
    }

    pub(crate) fn encode(&self, parent_id: &u32) -> String {
        format!(
            "{} 0 obj\n\
            <<\n\
            \x20\x20/Type /Page\n\
            \x20\x20/Parent {} 0 R\n\
            \x20\x20/MediaBox [0 0 {} {}]\n\
            >>\n\
            endobj\n\n",
            self.id.unwrap(),
            parent_id,
            self.size.0,
            self.size.1
        )
    }
}
