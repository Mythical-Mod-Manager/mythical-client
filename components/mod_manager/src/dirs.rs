use std::path::PathBuf;

use directories::ProjectDirs;

const APP_NAME: &str = "Mythical";

lazy_static! {
    static ref DIRS: ProjectDirs = ProjectDirs::from_path(PathBuf::from(APP_NAME)).unwrap();
}

pub fn get_config_dir() -> PathBuf {
    DIRS.data_local_dir().join("config")
}

pub fn get_cache_dir() -> PathBuf {
    DIRS.data_local_dir().join("cache")
}

pub fn get_binary_dir() -> PathBuf {
    DIRS.data_local_dir().join("cache")
}
