pub trait Object {
    fn get_id(&self) -> String;
    fn get_reference(&self) -> String {
        format!("{} 0 R", self.get_id())
    }
}
