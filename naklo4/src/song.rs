use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::value::{AddTag, SongTagValueStore, ValueType};

#[derive(Error, Debug, PartialEq)]
pub enum SongError {
    #[error("unsupported song: “{0}”")]
    UnsupportedSong(PathBuf),
}

type SongResult<T> = Result<T, SongError>;

fn mutagen_flac(path: &Path) {}

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
    fn from_single(path: &Path) -> SongResult<Song> {
        Ok(Song {
            path: path.to_path_buf(),
            mutagen_interaction: mutagen_interaction_closure_from(path)?,
            tags: SongTagValueStore::new(),
        })
    }

    pub fn from(paths: &Vec<PathBuf>) -> SongResult<Vec<Song>> {
        Ok(paths
            .iter()
            .map(|p| Song::from_single(p))
            .collect::<Result<_, _>>()?)
    }
}

impl AddTag for Song {
    fn add(&mut self, tag: &str, val: &ValueType) {
        self.tags.add(tag, val);
    }
}
