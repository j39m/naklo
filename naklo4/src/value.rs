// The defining feature of the intermediate “song” type is that it
// should have an `add(tag, val)` method. This can be expressed as a
// trait to provide a rendezvous for the control file and song modules.

use serde::Deserialize;

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
    fn add(&self, tag: &str, val: &ValueType) {
    }
}
