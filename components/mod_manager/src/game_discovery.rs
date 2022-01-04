use std::path::PathBuf;
use thiserror::Error;

use crate::game_config::GameConfigParseError;

/// A requirement to verify if a folder belongs to a specific game.
/// E.g. verify if a specific game executable is within the folder
#[derive(Debug)]
pub enum GameDiscoveryRequirement {
    /// A file exists with the this relative path.
    FileExists(PathBuf),

    /// At least one of the inner requirements are met
    Or(Vec<GameDiscoveryRequirement>),
    /// All of the inner requirements are met
    And(Vec<GameDiscoveryRequirement>),
}

/// An error describing why the game discovery of a folder failed
#[derive(Error, Debug)]
pub enum GameDiscoveryError {
    #[error("A filesystem error occured")]
    IOError(#[from] std::io::Error),
    #[error("Failed to parse the mythical config inside the game")]
    GameConfigParseError(#[from] GameConfigParseError),
}

type Result<T> = std::result::Result<T, GameDiscoveryError>;

impl GameDiscoveryRequirement {
    /// Verify if the folder is a game folder
    pub fn verify(&self, base_path: &PathBuf) -> Result<bool> {
        match self {
            GameDiscoveryRequirement::FileExists(path) => Ok(base_path.join(path).exists()),
            GameDiscoveryRequirement::Or(requirements) => {
                for req in requirements.iter() {
                    if req.verify(base_path)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            GameDiscoveryRequirement::And(requirements) => {
                for req in requirements.iter() {
                    if !req.verify(base_path)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }
    }
}
