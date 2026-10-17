// The defining feature of the intermediate “song” type is that it
// should have an `add(tag, val)` method. This can be expressed as a
// trait to provide a rendezvous for the control file and song modules.

use serde::Deserialize;
use std::collections::BTreeMap;

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

type StvInner = std::collections::HashMap<String, Vec<String>>;
#[derive(Debug, Clone, PartialEq)]
pub struct SongTagValueStore(pub StvInner);

impl SongTagValueStore {
    pub fn new() -> Self {
        Self(StvInner::new())
    }

    pub fn get(&self, tag: &str) -> Option<&Vec<String>> {
        self.0.get(tag)
    }
}

impl<const N: usize> From<[(String, Vec<String>); N]> for SongTagValueStore {
    fn from(arr: [(String, Vec<String>); N]) -> Self {
        Self(StvInner::from(arr))
    }
}

impl AddTag for SongTagValueStore {
    fn add(&mut self, tag: &str, val: &ValueType) {
        let val_vector: Vec<String> = match val {
            ValueType::Single(single) => vec![single.clone()],
            ValueType::Multiple(multiple) => multiple.clone(),
        };
        if !self.0.contains_key(tag) {
            self.0.insert(tag.to_string(), vec![]);
        }
        self.0
            .get_mut(tag)
            .expect("was inserted")
            .extend(val_vector);
    }
}

pub struct HomogeneousTagValueStore(pub BTreeMap<String, Vec<String>>);

impl HomogeneousTagValueStore {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}
