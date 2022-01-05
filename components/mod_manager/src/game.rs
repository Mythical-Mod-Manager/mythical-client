use std::{path::PathBuf, sync::Arc};

use crate::{
    config::{
        game::{parse_config_in_game, GameConfig},
        known_games::KnownGames,
        ConfigParseError,
    },
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
pub struct SupportedGames {
    games: Vec<Arc<GameMetadata>>,
}

impl SupportedGames {
    /// Create a new game library
    pub fn new(games: Vec<GameMetadata>) -> Self {
        SupportedGames {
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
    fn from_path(path: &PathBuf, metadata: Arc<GameMetadata>) -> Result<Self, ConfigParseError> {
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
    fn parse_in_path(path: &PathBuf) -> Result<Self, ConfigParseError> {
        let config = parse_config_in_game(path)?;
        match config {
            None => Ok(GameState::Unpatched),
            Some(config) => Ok(GameState::Patched(config)),
        }
    }
}

pub struct GameLibrary {
    games: Vec<Game>,
    known_games: KnownGames,
}

fn game_sorting_fn(g1: &Game, g2: &Game) -> std::cmp::Ordering {
    g1.metadata.name.cmp(&g2.metadata.name)
}

impl GameLibrary {
    pub fn init(supported_games: &SupportedGames) -> Self {
        let mut known_games = KnownGames::parse_or_default();
        let games = known_games.games.iter().filter_map(|path| {
            let discovered = supported_games.discover_folder(path);
            match discovered {
                Ok(Some(game)) => Some(game),
                Ok(None) => None,
                Err(_) => None, // Ignore errors, remove game
            }
        });

        let games = games.collect::<Vec<_>>();

        known_games.write().games = games.iter().map(|g| g.path.clone()).collect();

        GameLibrary { games, known_games }
    }

    fn normalize_from_games(&mut self) {
        self.games.sort_by(game_sorting_fn);
        self.known_games.write().games = self.games.iter().map(|g| g.path.clone()).collect();
    }

    pub fn games(&self) -> &[Game] {
        &self.games
    }

    pub fn add_game(&mut self, game: Game) {
        let existing = self.games.iter_mut().filter(|g| g.path == game.path).nth(0);
        if let Some(existing) = existing {
            *existing = game;
        } else {
            self.games.push(game);
        }
        self.normalize_from_games();
    }
}
