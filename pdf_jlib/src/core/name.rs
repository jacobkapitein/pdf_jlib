use std::hash::Hash;

use crate::traits::serialize::Serialize;

/// ISO 32000-1:2008 - 7.3.5 Name Objects
///
/// A name object is used to identify objects, for example, keys in a dictionary.
/// Name objects start with a slash when serialized, but the slash is not part of the name.
/// There is no need to provide a slash when creating a new Name object.
///
/// TODO: Implement more rules for name objects
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        let mut name = name.to_string();
        name = name.replace(" ", "#20");
        name = name.replace("#", "#23");

        Self {
            name: name.to_string(),
        }
    }
}

impl Serialize for Name {
    fn serialize(&self) -> Vec<u8> {
        // Append self.name with / prefix
        let mut serialized_output = "/".to_string();
        serialized_output.push_str(&self.name);
        serialized_output.into_bytes()
    }
}

impl Eq for Name {}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Name {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
