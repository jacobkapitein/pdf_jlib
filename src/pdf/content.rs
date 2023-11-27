pub struct Content {
    text: String,
    x: i32,
    y: i32,
}

impl Content {
    pub(crate) fn new(text: String, position: (i32, i32)) -> Self {
        Self {
            text,
            x: position.0,
            y: position.1,
        }
    }
}
