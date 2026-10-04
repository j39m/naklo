use std::range::RangeInclusive;

enum Span {
    SingleTrack(usize),
    GeneralSpan(String),
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
        return Some(RangeInclusive { start: 1, last: total_tracks });
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

fn parse_span(span: &Span, total_tracks: usize) -> Option<Vec<usize>> {
    match span {
        Span::SingleTrack(index) => Some(
            validate_range(
                RangeInclusive {
                    start: *index,
                    last: *index,
                },
                total_tracks,
            )?
            .into_iter()
            .collect(),
        ),
        Span::GeneralSpan(complicated) => {
            if let Some(ranges) = complicated
                .split_whitespace()
                .map(|token| parse_span_token(token, total_tracks))
                .collect::<Option<Vec<RangeInclusive<usize>>>>()
            {
                return Some(ranges.into_iter().flatten().collect());
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obvious_bad_cases() {
        assert_eq!(parse_span(&Span::GeneralSpan(String::from("hee")), 1), None);
        assert_eq!(parse_span(&Span::GeneralSpan(String::from("hee hee")), 1), None);

        assert_eq!(parse_span(&Span::SingleTrack(0), 1), None);
        assert_eq!(parse_span(&Span::SingleTrack(999), 1), None);
    }

    #[test]
    fn test_single_tracks() {
        assert_eq!(parse_span(&Span::SingleTrack(1), 1), Some(vec![1usize,]));
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1")), 1),
            Some(vec![1usize,])
        );
    }

    #[test]
    fn test_multiple_tracks() {
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1 2 4")), 4),
            Some(vec![1usize, 2, 4])
        );
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1-4")), 4),
            Some(vec![1usize, 2, 3, 4])
        );
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1-4 8 10")), 10),
            Some(vec![1usize, 2, 3, 4, 8, 10])
        );
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1-3 5 7-9")), 10),
            Some(vec![1usize, 2, 3, 5, 7, 8, 9])
        );
    }

    #[test]
    fn test_poking_out_of_bounds() {
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("1-4 6")), 5),
            None
        );
    }

    #[test]
    fn test_wildcard_span() {
        assert_eq!(
            parse_span(&Span::GeneralSpan(String::from("*")), 99),
            Some(RangeInclusive::from(1usize..=99).into_iter().collect())
        );
    }
}
