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
        Span::GeneralSpan(complicated) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obvious_bad_cases() {
        assert_eq!(parse_span(&Span::GeneralSpan(String::from("")), 1), None);
        assert_eq!(parse_span(&Span::GeneralSpan(String::from(" ")), 1), None);

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
