// Entries can be a list of key-value pairs, where the key is a string.
// The value can be anything, but for now we will support:
// - String
// - Dictionary
// - Object reference (As long as it has the Object trait)

use crate::{pdf::object::Object, traits::serialize::Serialize};
use std::collections::HashMap;

use super::name::Name;

pub struct Dictionary {
    pub id: u32,
    pub entries: HashMap<Name, Box<dyn Serialize>>,
}

impl Dictionary {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, key: Name, value: Box<dyn Serialize>) {
        self.entries.insert(key, value);
    }

    pub fn get_entry(&self, key: &Name) -> Option<&Box<dyn Serialize>> {
        self.entries.get(key)
    }

    pub fn get_entry_mut(&mut self, key: &Name) -> Option<&mut Box<dyn Serialize>> {
        self.entries.get_mut(key)
    }
}

impl Object for Dictionary {
    fn get_id(&self) -> String {
        format!("{} 0 obj", self.id)
    }
}

impl Serialize for Dictionary {
    fn serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();

        output.extend_from_slice(self.get_id().as_bytes());
        output.extend_from_slice(b"\n<<\n");
        for entry in &self.entries {
            output.extend_from_slice(b"\t");
            output.extend_from_slice(entry.0.serialize().as_slice());
            output.push(b' ');
            entry
                .1
                .serialize()
                .iter()
                .for_each(|byte| output.push(*byte));
            output.push(b'\n');
        }
        output.extend_from_slice(b">>\n");
        output.extend_from_slice(b"endobj\n\n");

        output
    }
}
