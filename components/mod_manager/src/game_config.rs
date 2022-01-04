use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

const GAME_CONFIG_FILE: &str = "mythical";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameConfig {
    patch_ver: String,
    selected_profile: Option<usize>, // If this is None or invalid, the loader assumes that the game is unpatched
    profiles: Vec<ModProfile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ModProfile {
    name: String,
    mods: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum GameConfigParseError {
    #[error("Couldn't deserialize the config")]
    DeserializeError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

fn parse_config_from_stream(
    stream: impl std::io::Read,
) -> Result<Option<GameConfig>, GameConfigParseError> {
    let unzipped = flate2::read::GzDecoder::new(stream);
    let config = serde_json::from_reader(unzipped)?;
    Ok(Some(config))
}

fn encode_config_to_stream(
    config: &GameConfig,
    stream: impl std::io::Write,
) -> Result<(), GameConfigParseError> {
    let zipped = flate2::write::GzEncoder::new(stream, flate2::Compression::default());
    serde_json::to_writer(zipped, config)?;
    Ok(())
}

pub fn parse_config_in_game(game: &PathBuf) -> Result<Option<GameConfig>, GameConfigParseError> {
    let path = game.join(GAME_CONFIG_FILE);
    let exists = path.exists();
    if !exists {
        return Ok(None);
    }

    let file = std::fs::File::open(path)?;
    parse_config_from_stream(file)
}

pub fn update_config_in_game(
    game: &PathBuf,
    config: &GameConfig,
) -> Result<(), GameConfigParseError> {
    let path = game.join(GAME_CONFIG_FILE);
    let exists = path.exists();
    if !exists {
        return Err(GameConfigParseError::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Game config file not found",
        )));
    }

    let file = std::fs::File::open(path)?;
    encode_config_to_stream(config, file)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_encode_decode_in_memory() {
        let mut profile = ModProfile {
            name: "test".to_string(),
            mods: HashMap::new(),
        };
        profile.mods.insert("test".to_string(), "1.0.0".to_string());
        profile
            .mods
            .insert("test2".to_string(), "1.2.0".to_string());
        profile
            .mods
            .insert("test3".to_string(), "1.5.0".to_string());

        let config = GameConfig {
            patch_ver: "1.0.0".to_string(),
            selected_profile: Some(0),
            profiles: vec![profile],
        };

        let mut buffer = Vec::new();
        encode_config_to_stream(&config, &mut buffer).unwrap();
        let decoded = parse_config_from_stream(Cursor::new(&buffer))
            .unwrap()
            .unwrap();
        assert_eq!(config, decoded);
    }
}
