use std::path::PathBuf;
use thiserror::Error;

use crate::game::{Game, GameLibrary};

#[derive(Error, Debug)]
pub enum SearchFolderError {
    #[error("A filesystem error occured")]
    IOError(#[from] std::io::Error),
    #[error("The folder does not exist")]
    FolderDoesNotExist,
    #[error("The path is not a folder")]
    NotAFolder,
}

/// Search a folder and discover games in child folders. Ignore all discovery errors.
pub fn search_folder<'a>(
    path: &PathBuf,
    library: &'a GameLibrary,
) -> Result<impl 'a + Iterator<Item = Game>, SearchFolderError> {
    let exists = path.exists();
    if !exists {
        return Err(SearchFolderError::FolderDoesNotExist);
    }

    let is_folder = path.is_dir();
    if !is_folder {
        return Err(SearchFolderError::NotAFolder);
    }

    let dir = std::fs::read_dir(path)?;

    let iter = dir.filter_map(move |entry| {
        let path = if let Ok(entry) = entry {
            entry.path()
        } else {
            return None;
        };

        if !path.is_dir() {
            return None;
        }

        let discovered = library.discover_folder(&path);
        if let Ok(discovered) = discovered {
            return discovered;
        } else {
            return None;
        }
    });

    Ok(iter)
}
