use thiserror::Error;
use std::path::PathBuf;
use std::path::Path;

#[derive(Error, Debug, PartialEq)]
pub enum SongError {
    #[error("unsupported song: “{0}”")]
    UnsupportedSong(PathBuf),
}

type SongResult<T> = Result<T, SongError>;

fn mutagen_flac(path: &Path) {
}

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
