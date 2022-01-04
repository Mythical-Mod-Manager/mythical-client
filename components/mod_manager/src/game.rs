use std::{path::PathBuf, sync::Arc};

use crate::{
    game_config::{parse_config_in_game, GameConfig, GameConfigParseError},
    game_discovery::{GameDiscoveryError, GameDiscoveryRequirement},
};

/// The representation of a game's metadata, such as the name, internal content folder, icon, etc.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct GameMetadata {
    /// Relative path for the content folder FactoryGame (inside of Satisfactory)
    pub contentpath_relative: String,

    /// The game name (e.g. Satisfactory)
    pub name: String,

    /// The game slug (e.g. satisfactory)
    pub internal_name: String,

    /// The manifest for the modloader which is often unique for each game
    #[derivative(Debug = "ignore")]
    pub modloader_manifest: String,

    /// The discovery requirements to identify if a folder belongs to this game
    pub discovery: GameDiscoveryRequirement,

    #[derivative(Debug = "ignore")]
    pub icon: (),
}

/// All known games that the mod manager can discover.
#[derive(Debug)]
pub struct GameLibrary {
    games: Vec<Arc<GameMetadata>>,
}

impl GameLibrary {
    /// Create a new game library
    pub fn new(games: Vec<GameMetadata>) -> Self {
        GameLibrary {
            games: games.into_iter().map(Arc::new).collect(),
        }
    }

    pub fn discover_folder(&self, path: &PathBuf) -> Result<Option<Game>, GameDiscoveryError> {
        for game in self.games.iter() {
            let discovered = game.discovery.verify(path)?;
            if discovered {
                let discovered_game = Game::from_path(path, game.clone())?;
                return Ok(Some(discovered_game));
            }
        }

        Ok(None)
    }
}

/// The representation of a game folder with some game metadata to display.
/// The game state contains mod manager related data.
#[derive(Debug)]
pub struct Game {
    // The absolute path of the game, e.g. F:\Steam\steamapps\common\Satisfactory
    path: PathBuf,
    metadata: Arc<GameMetadata>,
    state: GameState,
}

impl Game {
    fn from_path(
        path: &PathBuf,
        metadata: Arc<GameMetadata>,
    ) -> Result<Self, GameConfigParseError> {
        Ok(Game {
            path: path.clone(),
            metadata,
            state: GameState::parse_in_path(path)?,
        })
    }
}

#[derive(Debug)]
pub enum GameState {
    Unpatched,
    Patched(GameConfig),
}

impl GameState {
    fn parse_in_path(path: &PathBuf) -> Result<Self, GameConfigParseError> {
        let config = parse_config_in_game(path)?;
        match config {
            None => Ok(GameState::Unpatched),
            Some(config) => Ok(GameState::Patched(config)),
        }
    }
}
