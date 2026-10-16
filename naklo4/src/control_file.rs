use indexmap::IndexMap;
use serde::Deserialize;
use std::fs::File;
use std::path::PathBuf;
use std::range::RangeInclusive;
use thiserror::Error;

use crate::value::AddTag;
use crate::value::ValueType;

type SpanType = String;
type TagType = String;

// Use the New Type idiom to prevent the underlying IndexMaps from
// being literally the same types. If they were literal synonyms, Rust
// would forbid distinct implementations of `MapsToSongs`.
#[derive(Debug, PartialEq, Deserialize)]
struct SpanTagBlock(IndexMap<SpanType, IndexMap<TagType, ValueType>>);

#[derive(Debug, PartialEq, Deserialize)]
struct TagSpanBlock(IndexMap<TagType, IndexMap<SpanType, ValueType>>);

// `title-merge-block` has single-valued leaves.
#[derive(Debug, PartialEq, Deserialize)]
struct TitleMergeBlock(IndexMap<SpanType, String>);

#[derive(Debug, PartialEq, Deserialize)]
pub struct ControlFile {
    #[serde(rename = "span-tag-block")]
    span_tag_block: Option<SpanTagBlock>,
    #[serde(rename = "tag-span-block")]
    tag_span_block: Option<TagSpanBlock>,
    #[serde(rename = "title-merge-block")]
    title_merge_block: Option<TitleMergeBlock>,
}

#[derive(Error, Debug)]
pub enum ControlError {
    #[error("out of range: “{0}”")]
    SpanOutOfRange(usize),
    #[error("bad span: “{0}”")]
    BadSpan(String),
    #[error("unknown tag: “{0}”")]
    UnknownTag(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse failed in {0}")]
    Serde(String),
}

type ControlResult<T> = Result<T, ControlError>;

fn validate_range(
    range: RangeInclusive<usize>,
    total_tracks: usize,
) -> ControlResult<RangeInclusive<usize>> {
    if range.start < 1usize {
        return Err(ControlError::SpanOutOfRange(range.start));
    } else if range.last > total_tracks {
        return Err(ControlError::SpanOutOfRange(range.last));
    }

    Ok(range)
}

fn parse_span_token(token: &str, total_tracks: usize) -> ControlResult<RangeInclusive<usize>> {
    if token == "*" {
        return Ok(RangeInclusive {
            start: 1,
            last: total_tracks,
        });
    }
    if let Ok(single) = usize::from_str_radix(token, 10) {
        return validate_range(
            RangeInclusive {
                start: single,
                last: single,
            },
            total_tracks,
        );
    }
    if let Ok(split) = token
        .split('-')
        .map(|num| usize::from_str_radix(num, 10))
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
    {
        if split.len() == 2 {
            return validate_range(
                RangeInclusive {
                    start: split[0],
                    last: split[1],
                },
                total_tracks,
            );
        }
    }
    Err(ControlError::BadSpan(token.to_string()))
}

fn parse_span(span: &str, total_tracks: usize) -> ControlResult<Vec<usize>> {
    match span
        .split_whitespace()
        .map(|token| parse_span_token(token, total_tracks))
        .collect::<Result<Vec<RangeInclusive<usize>>, _>>()
    {
        Ok(ranges) => Ok(ranges.into_iter().flatten().collect()),
        Err(error) => Err(error),
    }
}

fn tag_is_valid(tag: &str) -> ControlResult<()> {
    crate::tag::tag_is_valid(tag).ok_or_else(|| ControlError::UnknownTag(tag.to_string()))
}

pub trait MapsToSongs {
    fn map_to<T: AddTag>(&self, songs: &mut Vec<T>) -> ControlResult<()>;
}

impl MapsToSongs for SpanTagBlock {
    fn map_to<T: AddTag>(&self, songs: &mut Vec<T>) -> ControlResult<()> {
        for (span_token, tags_and_values) in self.0.iter() {
            let span = parse_span(span_token.as_str(), songs.len())?;
            for (tag, val) in tags_and_values.iter() {
                tag_is_valid(tag)?;
                for index in span.iter() {
                    songs[index - 1].add(tag, val);
                }
            }
        }
        Ok(())
    }
}

impl MapsToSongs for TagSpanBlock {
    fn map_to<T: AddTag>(&self, songs: &mut Vec<T>) -> ControlResult<()> {
        for (tag, spans_and_values) in self.0.iter() {
            tag_is_valid(tag)?;
            for (span_token, val) in spans_and_values.iter() {
                let span = parse_span(span_token.as_str(), songs.len())?;
                for index in span.iter() {
                    songs[index - 1].add(tag, val);
                }
            }
        }
        Ok(())
    }
}

impl MapsToSongs for TitleMergeBlock {
    fn map_to<T: AddTag>(&self, songs: &mut Vec<T>) -> ControlResult<()> {
        let mut joined: Vec<Vec<String>> = vec![vec![]; songs.len()];
        for (span_token, title_fragment) in self.0.iter() {
            parse_span(span_token, songs.len())?
                .iter()
                .for_each(|index| joined[index - 1].push(title_fragment.clone()));
        }
        std::iter::zip(songs, joined).for_each(|(song, fragments)| {
            if !fragments.is_empty() {
                song.add("title", &ValueType::Single(fragments.join(" ")));
            }
        });
        Ok(())
    }
}

impl MapsToSongs for ControlFile {
    fn map_to<T: AddTag>(&self, songs: &mut Vec<T>) -> ControlResult<()> {
        if let Some(ref block) = self.span_tag_block {
            block.map_to(songs)?;
        }
        if let Some(ref block) = self.tag_span_block {
            block.map_to(songs)?;
        }
        if let Some(ref block) = self.title_merge_block {
            block.map_to(songs)?;
        }
        Ok(())
    }
}

pub fn from(paths: &Vec<PathBuf>) -> ControlResult<Vec<ControlFile>> {
    paths
        .iter()
        .map(|p| {
            Ok(serde_saphyr::from_reader::<_, ControlFile>(File::open(p)?)
                .map_err(|e| ControlError::Serde(format!("{}: {e}", p.display())))?)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::SongTagValueStore;
    use std::assert_matches;

    #[test]
    fn test_obvious_bad_spans() {
        assert_matches!(parse_span("hee", 1), Err(ControlError::BadSpan(_)));
        assert_matches!(parse_span("hee hee", 1), Err(ControlError::BadSpan(_)));

        assert_matches!(parse_span("0", 1), Err(ControlError::SpanOutOfRange(_)));
        assert_matches!(parse_span("999", 1), Err(ControlError::SpanOutOfRange(_)));
    }

    #[test]
    fn test_single_track_span() {
        assert_eq!(parse_span("1", 1).unwrap(), vec![1usize,]);
    }

    #[test]
    fn test_multiple_track_span() {
        assert_eq!(parse_span("1 2 4", 4).unwrap(), vec![1usize, 2, 4]);
        assert_eq!(parse_span("1-4", 4).unwrap(), vec![1usize, 2, 3, 4]);
        assert_eq!(
            parse_span("1-4 8 10", 10).unwrap(),
            vec![1usize, 2, 3, 4, 8, 10]
        );
        assert_eq!(
            parse_span("1-3 5 7-9", 10).unwrap(),
            vec![1usize, 2, 3, 5, 7, 8, 9]
        );
    }

    #[test]
    fn test_span_poking_out_of_bounds() {
        assert_matches!(parse_span("1-4 6", 5), Err(ControlError::SpanOutOfRange(_)));
    }

    #[test]
    fn test_wildcard_span() {
        assert_eq!(
            parse_span("*", 99).unwrap(),
            RangeInclusive::from(1usize..=99)
                .into_iter()
                .collect::<Vec<_>>()
        );
    }

    use indexmap::indexmap;

    #[test]
    fn test_basic_span_tag_block() {
        let yaml1 = r#"
        span-tag-block:
            "*":
                album: Hello there!
        "#;
        let control1: ControlFile = serde_saphyr::from_str(yaml1).unwrap();
        assert_eq!(
            control1,
            ControlFile {
                span_tag_block: Some(SpanTagBlock(indexmap! {
                    "*".to_string() => indexmap!{
                        "album".to_string() => ValueType::Single("Hello there!".to_string()),
                    },
                })),
                tag_span_block: None,
                title_merge_block: None,
            }
        );

        let yaml2 = r#"
        span-tag-block:
            "*":
                album:
                    - General
                    - Kenobi
        "#;
        let control2: ControlFile = serde_saphyr::from_str(yaml2).unwrap();
        assert_eq!(
            control2,
            ControlFile {
                span_tag_block: Some(SpanTagBlock(indexmap! {
                    "*".to_string() => indexmap!{
                        "album".to_string() =>
                            ValueType::Multiple(
                                vec!["General".to_string(), "Kenobi".to_string()]
                            ),
                    },
                })),
                tag_span_block: None,
                title_merge_block: None,
            }
        );
    }

    #[test]
    fn test_basic_tag_span_block() {
        let yaml1 = r#"
        tag-span-block:
            artist:
                01: General Grievous
                "02-05": Clanker Corps 48
        "#;
        let control1: ControlFile = serde_saphyr::from_str(yaml1).unwrap();
        assert_eq!(
            control1,
            ControlFile {
                span_tag_block: None,
                tag_span_block: Some(TagSpanBlock(indexmap! {
                    "artist".to_string() => indexmap!{
                        "01".to_string() => ValueType::Single("General Grievous".to_string()),
                        "02-05".to_string() => ValueType::Single("Clanker Corps 48".to_string()),
                    },
                })),
                title_merge_block: None,
            }
        );
    }

    #[test]
    fn test_span_tag_block_map_to() {
        let yaml = r#"
        01 02:
            composer: Fryderyk Chopin
        02-05:
            composer: Fred Showpan
            arranger: Bill Yell
        "#;
        let block: SpanTagBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 5];
        block.map_to(&mut songs).unwrap();
        assert_eq!(
            songs,
            vec![
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(),]
                )]),
                SongTagValueStore::from([
                    (
                        "composer".to_string(),
                        vec!["Fryderyk Chopin".to_string(), "Fred Showpan".to_string(),]
                    ),
                    ("arranger".to_string(), vec!["Bill Yell".to_string(),]),
                ]),
                SongTagValueStore::from([
                    ("composer".to_string(), vec!["Fred Showpan".to_string(),]),
                    ("arranger".to_string(), vec!["Bill Yell".to_string(),]),
                ]),
                SongTagValueStore::from([
                    ("composer".to_string(), vec!["Fred Showpan".to_string(),]),
                    ("arranger".to_string(), vec!["Bill Yell".to_string(),]),
                ]),
                SongTagValueStore::from([
                    ("composer".to_string(), vec!["Fred Showpan".to_string(),]),
                    ("arranger".to_string(), vec!["Bill Yell".to_string(),]),
                ]),
            ]
        );
    }

    #[test]
    fn test_span_tag_block_oob() {
        let yaml = r#"
        01 02:
            composer: Fryderyk Chopin
        02-05:
            composer: Fred Showpan
            arranger: Bill Yell
        "#;
        let block: SpanTagBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 4];
        assert_matches!(
            block.map_to(&mut songs),
            Err(ControlError::SpanOutOfRange(_))
        );
    }

    #[test]
    fn test_tag_span_block_map_to() {
        let yaml = r#"
        composer:
            "*": Fryderyk Chopin
            "02-04": Fred Showpan
            "05": Bill Yell
        "#;
        let block: TagSpanBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 5];
        block.map_to(&mut songs).unwrap();
        assert_eq!(
            songs,
            vec![
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(),]
                )]),
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(), "Fred Showpan".to_string(),]
                )]),
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(), "Fred Showpan".to_string(),]
                )]),
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(), "Fred Showpan".to_string(),]
                )]),
                SongTagValueStore::from([(
                    "composer".to_string(),
                    vec!["Fryderyk Chopin".to_string(), "Bill Yell".to_string(),]
                )]),
            ]
        );
    }

    #[test]
    fn test_tag_span_block_oob() {
        let yaml = r#"
        composer:
            "*": Fryderyk Chopin
            "02-04": Fred Showpan
            "05": Bill Yell
        "#;
        let block: TagSpanBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 4];
        assert_matches!(
            block.map_to(&mut songs),
            Err(ControlError::SpanOutOfRange(_))
        );
    }

    #[test]
    fn test_title_merge_block_map_to() {
        let yaml = r#"
        "01 02": hello
        "02-04": and
        05: goobai
        "#;
        let block: TitleMergeBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 5];
        block.map_to(&mut songs).unwrap();
        assert_eq!(
            songs,
            vec![
                SongTagValueStore::from([("title".to_string(), vec!["hello".to_string(),])]),
                SongTagValueStore::from([("title".to_string(), vec!["hello and".to_string(),])]),
                SongTagValueStore::from([("title".to_string(), vec!["and".to_string(),])]),
                SongTagValueStore::from([("title".to_string(), vec!["and".to_string(),])]),
                SongTagValueStore::from([("title".to_string(), vec!["goobai".to_string(),])]),
            ]
        );
    }

    #[test]
    fn test_title_merge_block_oob() {
        let yaml = r#"
        "01 02": hello
        "02-04": and
        05: goobai
        "#;
        let block: TitleMergeBlock = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 4];
        assert_matches!(
            block.map_to(&mut songs),
            Err(ControlError::SpanOutOfRange(_))
        );
    }

    #[test]
    fn test_title_stacking() {
        let yaml = r#"
        title-merge-block:
            "*": hello
            "03": and
        # serde_saphyr parses this before the above block, regardless
        # of its position here.
        tag-span-block:
            title:
                03: waku waku
        "#;
        let control: ControlFile = serde_saphyr::from_str(yaml).unwrap();
        let mut songs = vec![SongTagValueStore::new(); 3];
        control.map_to(&mut songs).unwrap();
        assert_eq!(
            songs,
            vec![
                SongTagValueStore::from([("title".to_string(), vec!["hello".to_string(),])]),
                SongTagValueStore::from([("title".to_string(), vec!["hello".to_string(),])]),
                SongTagValueStore::from([(
                    "title".to_string(),
                    // separate values
                    vec!["waku waku".to_string(), "hello and".to_string()]
                )]),
            ]
        );
    }
}
