pub mod dirs;
pub mod profile;

use std::{fs::create_dir_all, path::PathBuf};
use dirs::path_dir_storage;
pub use profile::Profile;


#[derive(Debug)]
pub enum Success {
    Cleared,
    Saved(Profile),
    Loaded(Profile),
    Removed(Vec<Profile>),
}


#[derive(Debug)]
pub enum Error {
    StorageNoPath,
    StorageNotDir,
    StorageCannotCreate(std::io::Error),
}


pub fn ensure_storage() -> Result<PathBuf, Error> {
    let path_dir = path_dir_storage().ok_or(Error::StorageNoPath)?;

    if path_dir.exists() {
        if path_dir.is_dir() {
            Ok(path_dir)
        } else {
            Err(Error::StorageNotDir)
        }
    } else {
        match create_dir_all(&path_dir) {
            Ok(()) => Ok(path_dir),
            Err(e) => Err(Error::StorageCannotCreate(e)),
        }
    }
}


pub fn profile_clear() -> Result<Success, Error> {
    todo!()
}


pub fn profile_save(name: String, clobber: bool) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;

    todo!()
}


pub fn profile_load(name: String) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;

    todo!()
}


pub fn profile_remove(name: Vec<String>, confirm: bool) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;

    todo!()
}
