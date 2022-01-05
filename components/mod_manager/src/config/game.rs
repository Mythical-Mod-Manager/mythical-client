use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use super::{encode_config_to_stream, parse_config_from_stream, ConfigParseError};

const GAME_CONFIG_FILE: &str = "mythical";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameConfig {
    pub patch_ver: String,
    pub selected_profile: Option<usize>, // If this is None or invalid, the loader assumes that the game is unpatched
    pub profiles: Vec<ModProfile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ModProfile {
    pub name: String,
    pub mods: HashMap<String, String>,
}

pub fn parse_config_in_game(game: &PathBuf) -> Result<Option<GameConfig>, ConfigParseError> {
    let path = game.join(GAME_CONFIG_FILE);
    let exists = path.exists();
    if !exists {
        return Ok(None);
    }

    let file = std::fs::File::open(path)?;
    parse_config_from_stream(file)
}

pub fn update_config_in_game(game: &PathBuf, config: &GameConfig) -> Result<(), ConfigParseError> {
    let path = game.join(GAME_CONFIG_FILE);
    let exists = path.exists();
    if !exists {
        return Err(ConfigParseError::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Game config file not found",
        )));
    }

    let file = std::fs::File::open(path)?;
    encode_config_to_stream(config, file)
}
