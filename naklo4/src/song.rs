use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fmt;
use thiserror::Error;

use crate::value::{AddTag, HomogeneousTagValueStore, SongTagValueStore, ValueType};

#[derive(Error, Debug, PartialEq)]
pub enum SongError {
    #[error("unsupported song: “{0}”")]
    UnsupportedSong(PathBuf),
    #[error("dubious title: “{0}”")]
    DubiousTitle(String),
    #[error("unpretentious single quote: “{0}”")]
    SingleQuote(String),
    #[error("unpretentious double quote: “{0}”")]
    DoubleQuote(String),
    #[error("unbalanced quotes: “{0}”")]
    UnbalancedQuotes(String),
    #[error("tag “{0}” not homogeneous")]
    TagNotHomogeneous(String),
}

type SongResult<T> = Result<T, SongError>;

fn mutagen_flac(path: &Path) {}

fn mutagen_dummy(_path: &Path) {}

fn mutagen_interaction_closure_from(path: &Path) -> SongResult<fn(&Path)> {
    let Some(extension_osstr) = path.extension() else {
        return Err(SongError::UnsupportedSong(path.to_path_buf()));
    };
    let Some(extension) = extension_osstr.to_str() else {
        return Err(SongError::UnsupportedSong(path.to_path_buf()));
    };
    match extension {
        "flac" => Ok(mutagen_flac),
        _ => Err(SongError::UnsupportedSong(path.to_path_buf())),
    }
}

pub struct Song {
    path: PathBuf,
    mutagen_interaction: fn(&Path),
    tags: SongTagValueStore,
}

impl Song {
    fn from(path: &Path) -> SongResult<Self> {
        Ok(Self {
            path: path.to_path_buf(),
            mutagen_interaction: mutagen_interaction_closure_from(path)?,
            tags: SongTagValueStore::new(),
        })
    }

    fn for_testing() -> Self {
        Self {
            path: PathBuf::new(),
            mutagen_interaction: mutagen_dummy,
            tags: SongTagValueStore::new(),
        }
    }

    fn get_title(&self) -> Option<&str> {
        if let Some(titles) = self.tags.get("title") {
            return Some(&titles[0]);
        }
        None
    }

    fn lint_unbalanced_quotes_in_title(title: &str, left: char, right: char) -> SongResult<()> {
        (title.split(left).count() == title.split(right).count())
            .ok_or_else(|| SongError::UnbalancedQuotes(title.to_string()))
    }

    fn lint_title(&self) -> SongResult<()> {
        let Some(title) = self.get_title() else {
            return Err(SongError::DubiousTitle(String::from("[no title]")));
        };
        if title.is_empty() {
            return Err(SongError::DubiousTitle(String::from("[no title]")));
        }
        if title.find('\'').is_some() {
            return Err(SongError::SingleQuote(title.to_string()));
        }
        if title.find('"').is_some() {
            return Err(SongError::DoubleQuote(title.to_string()));
        }
        for (left, right) in [('「', '」'), ('«', '»'), ('“', '”'), ('‘', '’')] {
            Self::lint_unbalanced_quotes_in_title(title, left, right)?;
        }
        Ok(())
    }
}

impl AddTag for Song {
    fn add(&mut self, tag: &str, val: &ValueType) {
        self.tags.add(tag, val);
    }
}

pub fn from(paths: &Vec<PathBuf>) -> SongResult<Vec<Song>> {
    Ok(paths
        .iter()
        .map(|p| Song::from(p))
        .collect::<Result<_, _>>()?)
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "《{}》\n", self.get_title().unwrap_or("[NO TITLE]"))?;
        write!(f, "{}", self.tags)
    }
}

pub struct LintResult {
    pub title_lints: Vec<SongError>,
    pub homogeneity_lints: Vec<SongError>,
    pub homogeneous_tags: HomogeneousTagValueStore,
}

impl LintResult {
    pub fn from(songs: &Vec<Song>) -> Self {
        let mut title_lints: Vec<SongResult<()>> = Vec::new();
        let mut nonhomogeneous_tags: HashSet<&str> = HashSet::new();

        // Hierarchy of tags => [values => counts]. A tag is homogeneous
        // when (for each value) counts == total_songs.
        //
        // In theory we _could_ get away with simply mapping
        // tags => values and assert that the collected length is
        // 0 % total_songs, but there are weird silly corner cases —
        // for example, `album` (rarely) and `genre` (sometimes) can
        // take multiple values. Not keeping a strict count means one
        // could construct a basket of values that satisfy this without
        // actually being homogeneous.
        //
        // For example, imagine an erroneously tagged album of 13 songs.
        // 1 song has 13 `album` values, and the other 12 have zero.
        let mut tags_to_values_to_counts: HashMap<&str, HashMap<&Vec<String>, usize>> =
            HashMap::new();
        for song in songs {
            title_lints.push(song.lint_title());
            for (tag, values) in song
                .tags
                .0
                .iter()
                .filter(|(t, _v)| crate::tag::tag_is_homogeneous(t))
            {
                *(*tags_to_values_to_counts
                    .entry(tag)
                    .or_insert(HashMap::new()))
                .entry(values)
                .or_insert(0) += 1;
            }
        }
        let mut homogeneous_tags = HomogeneousTagValueStore::new();
        for (tag, map) in tags_to_values_to_counts {
            for (value, count) in map.iter() {
                if *count != songs.len() {
                    nonhomogeneous_tags.insert(tag);
                }
            }
            homogeneous_tags
                .0
                .insert(tag.to_string(), map.iter().next().unwrap().0.to_vec());
        }
        Self {
            title_lints: title_lints.into_iter().filter_map(|r| r.err()).collect(),
            homogeneity_lints: nonhomogeneous_tags
                .into_iter()
                .map(|t| SongError::TagNotHomogeneous(t.to_string()))
                .collect(),
            homogeneous_tags: homogeneous_tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn empty_title_is_dubious() {
        let mut no_title = Song::for_testing();
        assert_matches!(no_title.lint_title(), Err(SongError::DubiousTitle(_)));
    }

    #[test]
    fn any_ascii_quotes_are_dubious() {
        let mut title_with_single_quote = Song::for_testing();
        title_with_single_quote.add("title", &ValueType::Single(String::from(r#""a""#)));
        assert_matches!(
            title_with_single_quote.lint_title(),
            Err(SongError::DoubleQuote(_))
        );

        let mut title_with_double_quotes = Song::for_testing();
        title_with_double_quotes.add("title", &ValueType::Single(String::from(r#"'a'"#)));
        assert_matches!(
            title_with_double_quotes.lint_title(),
            Err(SongError::SingleQuote(_))
        );
    }

    #[test]
    fn unbalanced_quotes_are_dubious() {
        let mut title_with_unbalanced_left_ja_quote = Song::for_testing();
        title_with_unbalanced_left_ja_quote.add("title", &ValueType::Single(String::from(r#"「"#)));
        assert_matches!(
            title_with_unbalanced_left_ja_quote.lint_title(),
            Err(SongError::UnbalancedQuotes(_))
        );
        let mut title_with_unbalanced_right_ja_quote = Song::for_testing();
        title_with_unbalanced_right_ja_quote
            .add("title", &ValueType::Single(String::from(r#"」"#)));
        assert_matches!(
            title_with_unbalanced_right_ja_quote.lint_title(),
            Err(SongError::UnbalancedQuotes(_))
        );

        let mut title_with_unbalanced_whatsitcalled_left_quote = Song::for_testing();
        title_with_unbalanced_whatsitcalled_left_quote
            .add("title", &ValueType::Single(String::from(r#"»"#)));
        assert_matches!(
            title_with_unbalanced_whatsitcalled_left_quote.lint_title(),
            Err(SongError::UnbalancedQuotes(_))
        );
        let mut title_with_unbalanced_whatsitcalled_right_quote = Song::for_testing();
        title_with_unbalanced_whatsitcalled_right_quote
            .add("title", &ValueType::Single(String::from(r#"«"#)));
        assert_matches!(
            title_with_unbalanced_whatsitcalled_right_quote.lint_title(),
            Err(SongError::UnbalancedQuotes(_))
        );
    }

    #[test]
    fn balanced_quotes_are_ok() {
        let mut title_with_balanced_double_quotes = Song::for_testing();
        title_with_balanced_double_quotes
            .add("title", &ValueType::Single(String::from(r#"““”“””"#)));
        assert!(title_with_balanced_double_quotes.lint_title().is_ok());

        let mut title_with_balanced_ja_quotes = Song::for_testing();
        title_with_balanced_ja_quotes.add(
            "title",
            &ValueType::Single(String::from(r#"「」「「」「」」」「"#)),
        );
        assert!(title_with_balanced_ja_quotes.lint_title().is_ok());
    }

    #[test]
    fn empty_songs_are_homogeneous() {
        let empty = vec![Song::for_testing(), Song::for_testing()];
        let lints = LintResult::from(&empty).homogeneity_lints;
        assert_eq!(lints.len(), 0);
    }

    #[test]
    fn same_albums_are_homogeneous() {
        let mut songs = vec![Song::for_testing(), Song::for_testing()];
        songs[0].add("album", &ValueType::Single("general".to_string()));
        songs[1].add("album", &ValueType::Single("general".to_string()));
        let lints = LintResult::from(&songs).homogeneity_lints;
        assert_eq!(lints.len(), 0);
    }

    #[test]
    fn different_albums_are_not_homogeneous() {
        let mut songs = vec![Song::for_testing(), Song::for_testing()];
        songs[0].add("album", &ValueType::Single("general".to_string()));
        songs[1].add(
            "album",
            &ValueType::Multiple(vec!["general".to_string(), "kenobi".to_string()]),
        );
        let lints = LintResult::from(&songs).homogeneity_lints;
        assert_eq!(lints.len(), 1);
        assert_matches!(lints[0], SongError::TagNotHomogeneous(_));
    }

    #[test]
    fn heterogeneous_tags_can_differ() {
        let mut songs = vec![Song::for_testing(), Song::for_testing()];
        songs[0].add("title", &ValueType::Single("general".to_string()));
        songs[1].add("title", &ValueType::Single("kenobi".to_string()));
        let lints = LintResult::from(&songs).homogeneity_lints;
        assert_eq!(lints.len(), 0);
    }
}
