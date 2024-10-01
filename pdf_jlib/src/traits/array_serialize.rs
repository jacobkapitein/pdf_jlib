pub trait ArraySerialize {
    fn serialize_array(&self) -> Vec<u8>;
}

impl ArraySerialize for String {
    fn serialize_array(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ArraySerialize for &str {
    fn serialize_array(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ArraySerialize for i32 {
    fn serialize_array(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}
