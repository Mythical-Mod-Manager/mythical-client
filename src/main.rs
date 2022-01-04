use std::path::PathBuf;

use mod_manager::{
    game::{GameLibrary, GameMetadata},
    game_discovery::GameDiscoveryRequirement,
    game_search::search_folder,
};

fn get_pavlov_metadata() -> GameMetadata {
    let pavlov_discovery = GameDiscoveryRequirement::And(vec![
        GameDiscoveryRequirement::FileExists(PathBuf::from(
            r"Pavlov\Binaries\Win64\Pavlov-Win64-Shipping.exe",
        )),
        GameDiscoveryRequirement::FileExists(PathBuf::from("Pavlov.exe")),
    ]);

    GameMetadata {
        contentpath_relative: "Pavlov".to_string(),
        name: "Pavlov".to_string(),
        internal_name: "pavlov".to_string(),
        modloader_manifest: "todo".to_string(),
        discovery: pavlov_discovery,
        icon: (),
    }
}

fn get_it_takes_two_metadata() -> GameMetadata {
    let it_takes_two_discovery =
        GameDiscoveryRequirement::And(vec![GameDiscoveryRequirement::FileExists(PathBuf::from(
            r"Nuts\Binaries\Win64\ItTakesTwo.exe",
        ))]);

    GameMetadata {
        contentpath_relative: "ItTakesTwo".to_string(),
        name: "It Takes Two".to_string(),
        internal_name: "it_takes_two".to_string(),
        modloader_manifest: "todo".to_string(),
        discovery: it_takes_two_discovery,
        icon: (),
    }
}

#[tokio::main]
async fn main() {
    let games = vec![get_pavlov_metadata(), get_it_takes_two_metadata()];

    let library = GameLibrary::new(games);

    let items = search_folder(&PathBuf::from(r"F:\Steam\steamapps\common"), &library).unwrap();
    let items = items.collect::<Vec<_>>();

    dbg!(items);
}
