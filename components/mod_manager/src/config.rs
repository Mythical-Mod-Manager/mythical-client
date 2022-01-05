use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub mod game;
pub mod known_games;
pub mod settings;

#[derive(Error, Debug)]
pub enum ConfigParseError {
    #[error("Couldn't deserialize the config")]
    DeserializeError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

fn parse_config_from_stream<T: DeserializeOwned>(
    stream: impl std::io::Read,
) -> Result<T, ConfigParseError> {
    // let unzipped = flate2::read::GzDecoder::new(stream);
    // let config = serde_json::from_reader(unzipped)?;
    let config = serde_json::from_reader(stream)?;
    Ok(config)
}

fn encode_config_to_stream<T: Serialize>(
    config: &T,
    stream: impl std::io::Write,
) -> Result<(), ConfigParseError> {
    // let zipped = flate2::write::GzEncoder::new(stream, flate2::Compression::default());
    // serde_json::to_writer(zipped, config)?;
    serde_json::to_writer(stream, config)?;
    Ok(())
}

fn encode_config_to_file<T: Serialize>(config: &T, path: &PathBuf) -> Result<(), ConfigParseError> {
    std::fs::create_dir_all(path.parent().unwrap())?;
    let file = std::fs::File::create(path)?;
    encode_config_to_stream(config, file)?;
    Ok(())
}

fn parse_config_from_file<T: DeserializeOwned>(path: &PathBuf) -> Result<T, ConfigParseError> {
    let file = std::fs::File::open(path)?;
    parse_config_from_stream(file)
}

/// Helps represent a config structure that saves each time a mutable guard is dropped.
pub struct SaveOnWrite<T: DeserializeOwned + Serialize> {
    inner: T,
    path: PathBuf,
}

impl<T: DeserializeOwned + Serialize> SaveOnWrite<T> {
    fn parse(path: PathBuf) -> Result<SaveOnWrite<T>, ConfigParseError> {
        parse_config_from_file(&path).map(|val| SaveOnWrite::new(val, path))
    }

    fn parse_or_default(path: PathBuf) -> SaveOnWrite<T>
    where
        T: Default,
    {
        match Self::parse(path.clone()) {
            Ok(config) => config,
            Err(_) => SaveOnWrite::new(T::default(), path),
        }
    }

    fn persist(&self) -> Result<(), ConfigParseError> {
        encode_config_to_file(&self.inner, &self.path)?;
        Ok(())
    }

    fn new(inner: T, path: PathBuf) -> Self {
        Self { inner, path }
    }

    pub fn write<'a>(&'a mut self) -> SOWWriteGuard<'a, T> {
        SOWWriteGuard(self)
    }
}

impl<T: DeserializeOwned + Serialize> std::ops::Deref for SaveOnWrite<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

pub struct SOWWriteGuard<'a, T: DeserializeOwned + Serialize>(&'a mut SaveOnWrite<T>);

impl<T: DeserializeOwned + Serialize> std::ops::Deref for SOWWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.inner
    }
}

impl<T: DeserializeOwned + Serialize> std::ops::DerefMut for SOWWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0.inner
    }
}

impl<T: DeserializeOwned + Serialize> Drop for SOWWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.0.persist().ok();
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::Cursor};

    use crate::config::game::{GameConfig, ModProfile};

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
        let decoded = parse_config_from_stream(Cursor::new(&buffer)).unwrap();
        assert_eq!(config, decoded);
    }
}
