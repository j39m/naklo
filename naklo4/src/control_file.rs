use indexmap::IndexMap;
use serde::Deserialize;
use std::range::RangeInclusive;
use thiserror::Error;

use crate::value::ValueType;

type SpanType = String;
type TagType = String;

#[derive(Debug, PartialEq, Deserialize)]
struct ControlFile {
    #[serde(rename = "span-tag-block")]
    span_tag_block: Option<IndexMap<SpanType, IndexMap<TagType, ValueType>>>,
    #[serde(rename = "tag-span-block")]
    tag_span_block: Option<IndexMap<TagType, IndexMap<SpanType, ValueType>>>,
}

#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    #[error("out of range: “{0}”")]
    SpanOutOfRange(usize),
    #[error("bad span: “{0}”")]
    BadSpan(String),
}

fn validate_range(
    range: RangeInclusive<usize>,
    total_tracks: usize,
) -> Result<RangeInclusive<usize>, ControlError> {
    if range.start < 1usize {
        return Err(ControlError::SpanOutOfRange(range.start));
    } else if range.last > total_tracks {
        return Err(ControlError::SpanOutOfRange(range.last));
    }

    Ok(range)
}

fn parse_span_token(
    token: &str,
    total_tracks: usize,
) -> Result<RangeInclusive<usize>, ControlError> {
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

fn parse_span(span: &str, total_tracks: usize) -> Result<Vec<usize>, ControlError> {
    match span
        .split_whitespace()
        .map(|token| parse_span_token(token, total_tracks))
        .collect::<Result<Vec<RangeInclusive<usize>>, _>>()
    {
        Ok(ranges) => Ok(ranges.into_iter().flatten().collect()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(parse_span("1", 1), Ok(vec![1usize,]));
    }

    #[test]
    fn test_multiple_track_span() {
        assert_eq!(parse_span("1 2 4", 4), Ok(vec![1usize, 2, 4]));
        assert_eq!(parse_span("1-4", 4), Ok(vec![1usize, 2, 3, 4]));
        assert_eq!(parse_span("1-4 8 10", 10), Ok(vec![1usize, 2, 3, 4, 8, 10]));
        assert_eq!(
            parse_span("1-3 5 7-9", 10),
            Ok(vec![1usize, 2, 3, 5, 7, 8, 9])
        );
    }

    #[test]
    fn test_span_poking_out_of_bounds() {
        assert_matches!(parse_span("1-4 6", 5), Err(ControlError::SpanOutOfRange(_)));
    }

    #[test]
    fn test_wildcard_span() {
        assert_eq!(
            parse_span("*", 99),
            Ok(RangeInclusive::from(1usize..=99).into_iter().collect())
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
                span_tag_block: Some(indexmap! {
                    "*".to_string() => indexmap!{
                        "album".to_string() => ValueType::Single("Hello there!".to_string()),
                    },
                }),
                tag_span_block: None,
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
                span_tag_block: Some(indexmap! {
                    "*".to_string() => indexmap!{
                        "album".to_string() =>
                            ValueType::Multiple(
                                vec!["General".to_string(), "Kenobi".to_string()]
                            ),
                    },
                }),
                tag_span_block: None,
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
                tag_span_block: Some(indexmap! {
                    "artist".to_string() => indexmap!{
                        "01".to_string() => ValueType::Single("General Grievous".to_string()),
                        "02-05".to_string() => ValueType::Single("Clanker Corps 48".to_string()),
                    },
                }),
            }
        );
    }
}
