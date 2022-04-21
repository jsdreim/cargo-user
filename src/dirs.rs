use std::{env::var_os, path::PathBuf};
use directories::{ProjectDirs, UserDirs};


const DIR_CARGO: &str = ".cargo";
const FILE_CARGO_CREDENTIALS: &str = "credentials";
const VAR_CARGO_HOME: &str = "CARGO_HOME";


pub fn path_dir_cargo() -> Option<PathBuf> {
    match var_os(VAR_CARGO_HOME) {
        Some(path_str) => Some(PathBuf::from(path_str)),
        None => {
            let mut path: PathBuf = UserDirs::new()?.home_dir().to_owned();
            path.push(DIR_CARGO);
            Some(path)
        }
    }
}


pub fn path_dir_config() -> Option<PathBuf> {
    Some(ProjectDirs::from("", "", "cargo-user")?.config_dir().to_owned())
}


pub fn path_dir_storage() -> Option<PathBuf> {
    let mut path = path_dir_config()?;
    path.push("profiles");
    Some(path)
}


pub fn path_file_credentials() -> Option<PathBuf> {
    let mut path = path_dir_cargo()?;
    path.push(FILE_CARGO_CREDENTIALS);
    Some(path)
}
