use indexmap::IndexMap;
use serde::Deserialize;
use std::range::RangeInclusive;

type SpanType = String;
type TagType = String;

// The `#[serde(untagged)]` is load-bearing here. Without this, serde
// would expect the variant names directly in the YAML. Something like:
//
// ```
// album:
//   Single: Hello there!
// ```
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum ValueType {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, PartialEq, Deserialize)]
struct ControlFile {
    #[serde(rename = "span-tag-block")]
    span_tag_block: Option<IndexMap<SpanType, IndexMap<TagType, ValueType>>>,
    #[serde(rename = "tag-span-block")]
    tag_span_block: Option<IndexMap<TagType, IndexMap<SpanType, ValueType>>>,
}

fn validate_range(
    range: RangeInclusive<usize>,
    total_tracks: usize,
) -> Option<RangeInclusive<usize>> {
    if range.start < 1usize || range.last > total_tracks {
        return None;
    }

    Some(range)
}

fn parse_span_token(token: &str, total_tracks: usize) -> Option<RangeInclusive<usize>> {
    if token == "*" {
        return Some(RangeInclusive {
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
    None
}

fn parse_span(span: &str, total_tracks: usize) -> Option<Vec<usize>> {
    if let Some(ranges) = span
        .split_whitespace()
        .map(|token| parse_span_token(token, total_tracks))
        .collect::<Option<Vec<RangeInclusive<usize>>>>()
    {
        return Some(ranges.into_iter().flatten().collect());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obvious_bad_cases() {
        assert_eq!(parse_span("hee", 1), None);
        assert_eq!(parse_span("hee hee", 1), None);

        assert_eq!(parse_span("0", 1), None);
        assert_eq!(parse_span("999", 1), None);
    }

    #[test]
    fn test_single_tracks() {
        assert_eq!(parse_span("1", 1), Some(vec![1usize,]));
    }

    #[test]
    fn test_multiple_tracks() {
        assert_eq!(parse_span("1 2 4", 4), Some(vec![1usize, 2, 4]));
        assert_eq!(parse_span("1-4", 4), Some(vec![1usize, 2, 3, 4]));
        assert_eq!(
            parse_span("1-4 8 10", 10),
            Some(vec![1usize, 2, 3, 4, 8, 10])
        );
        assert_eq!(
            parse_span("1-3 5 7-9", 10),
            Some(vec![1usize, 2, 3, 5, 7, 8, 9])
        );
    }

    #[test]
    fn test_poking_out_of_bounds() {
        assert_eq!(parse_span("1-4 6", 5), None);
    }

    #[test]
    fn test_wildcard_span() {
        assert_eq!(
            parse_span("*", 99),
            Some(RangeInclusive::from(1usize..=99).into_iter().collect())
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
