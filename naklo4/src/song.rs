use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::value::{AddTag, SongTagValueStore, ValueType};

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
}
