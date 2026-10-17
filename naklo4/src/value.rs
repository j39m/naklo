// The defining feature of the intermediate “song” type is that it
// should have an `add(tag, val)` method. This can be expressed as a
// trait to provide a rendezvous for the control file and song modules.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;

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

const MULTIVAL_JOINER: &str = "\n        - ";
fn write_value_after_tag(val: &Vec<String>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match val.len() {
        1 => write!(f, " {}", val[0]),
        _ => write!(f, "{MULTIVAL_JOINER}{}", val.join(MULTIVAL_JOINER)),
    }
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
        self.0
            .entry(tag.to_string())
            .or_insert(vec![])
            .extend(val_vector);
    }
}

pub struct HomogeneousTagValueStore(pub BTreeMap<String, Vec<String>>);

impl HomogeneousTagValueStore {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl fmt::Display for HomogeneousTagValueStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Homogeneous tags:\n")?;
        for (tag, values) in self.0.iter() {
            write!(f, "    {tag:20}:")?;
            write_value_after_tag(&values, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}
