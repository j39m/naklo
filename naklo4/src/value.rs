// The defining feature of the intermediate “song” type is that it
// should have an `add(tag, val)` method. This can be expressed as a
// trait to provide a rendezvous for the control file and song modules.

use serde::Deserialize;
use std::collections::HashMap;

// The `#[serde(untagged)]` is load-bearing here. Without this, serde
// would expect the variant names directly in the YAML. Something like:
//
// ```
// album:
//   Single: Hello there!
// ```
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ValueType {
    Single(String),
    Multiple(Vec<String>),
}

pub trait AddTag {
    fn add(&mut self, tag: &str, val: &ValueType);
}

pub type SongTagValueStore = HashMap<String, Vec<String>>;

impl AddTag for SongTagValueStore {
    fn add(&mut self, tag: &str, val: &ValueType) {
        let val_vector: Vec<String> = match val {
            ValueType::Single(single) => vec![single.clone()],
            ValueType::Multiple(multiple) => multiple.clone(),
        };
        if !self.contains_key(tag) {
            self.insert(tag.to_string(), vec![]);
        }
        self.get_mut(tag).expect("was inserted").extend(val_vector);
    }
}
