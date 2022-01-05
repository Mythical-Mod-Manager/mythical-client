use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::dirs::get_config_dir;

use super::SaveOnWrite;

const KNOWN_GAMES_CONFIG: &str = "known_games";

fn get_path() -> PathBuf {
    get_config_dir().join(KNOWN_GAMES_CONFIG)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KnownGamesData {
    pub games: Vec<PathBuf>,
}

impl Default for KnownGamesData {
    fn default() -> Self {
        KnownGamesData { games: Vec::new() }
    }
}

pub struct KnownGames(SaveOnWrite<KnownGamesData>);

impl KnownGames {
    pub fn parse_or_default() -> KnownGames {
        let path = get_path();
        KnownGames(SaveOnWrite::parse_or_default(path))
    }
}

impl std::ops::Deref for KnownGames {
    type Target = SaveOnWrite<KnownGamesData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for KnownGames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
