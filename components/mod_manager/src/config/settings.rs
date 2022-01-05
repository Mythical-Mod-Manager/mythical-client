use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::dirs::get_config_dir;

use super::{
    encode_config_to_file, encode_config_to_stream, parse_config_from_file,
    parse_config_from_stream, ConfigParseError, SaveOnWrite,
};

const SETTINGS_CONFIG: &str = "settings";

fn get_path() -> PathBuf {
    get_config_dir().join(SETTINGS_CONFIG)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MythicalSettingsData {
    pub discover_on_launch: bool,
}

impl Default for MythicalSettingsData {
    fn default() -> Self {
        MythicalSettingsData {
            discover_on_launch: true,
        }
    }
}

pub struct MythicalSettings(SaveOnWrite<MythicalSettingsData>);

impl MythicalSettings {
    pub fn parse_or_default() -> MythicalSettings {
        let path = get_path();
        MythicalSettings(SaveOnWrite::parse_or_default(path))
    }
}

impl std::ops::Deref for MythicalSettings {
    type Target = SaveOnWrite<MythicalSettingsData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MythicalSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
