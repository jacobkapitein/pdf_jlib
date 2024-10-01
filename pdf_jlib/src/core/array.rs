use crate::traits::{array_serialize::ArraySerialize, serialize::Serialize};

/// ISO 32000-1:2008 - 7.3.6 Array Objects
///
/// An array may contain any objects of any type.
/// Numbers, strings, dictionaries, any other objects including other arrays.
/// It may have 0 items.
///
/// The PDF spec only supports one-dimensional arrays,
/// but higher dimensions can be simulated by just adding nested arrays.
pub struct Array {
    pub elements: Vec<Box<dyn ArraySerialize>>,
}

impl Array {
    pub fn new(elements: Vec<Box<dyn ArraySerialize>>) -> Self {
        Self { elements }
    }

    pub fn add_element(&mut self, element: Box<dyn ArraySerialize>) {
        self.elements.push(element);
    }
}

impl Serialize for Array {
    fn serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();

        output.push(b'[');
        output.push(b' ');
        for element in &self.elements {
            output.extend_from_slice(element.serialize_array().as_slice());
            output.push(b' ');
        }
        output.push(b']');

        output
    }
}
