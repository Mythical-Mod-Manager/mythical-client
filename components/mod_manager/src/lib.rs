use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use config::{known_games::KnownGames, settings::MythicalSettings};
use game::{Game, SupportedGames};
use task_guard::TaskGuard;

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate lazy_static;

pub mod config;
mod dirs;
pub mod game;
pub mod game_discovery;
pub mod game_search;
mod task_guard;

pub struct ModManager(Arc<Mutex<ModManagerInner>>);

pub struct ModManagerInner {
    settings: MythicalSettings,
    guard: TaskGuard<PathBuf>,
}

pub struct ModManagerGames {}

pub enum ModManagerState {
    FetchingMetadata,
    DiscoveringGames {
        library: SupportedGames,
    },
    Ready {
        library: SupportedGames,
        known_games: KnownGames,
        games: Vec<Game>,
    },
}
