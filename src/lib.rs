pub mod dirs;
pub mod profile;

use std::{fs::create_dir_all, path::PathBuf};
use dirs::{path_dir_storage, path_file_credentials};
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
    CredentialsNoPath,
    CredentialsNotFound,
    CredentialsCannotRead(std::io::Error),
    CredentialsCannotWrite(std::io::Error),

    ProfileExists(Profile),
    ProfileNoPath,
    ProfileNotFound,
    ProfileCannotRead(std::io::Error),
    ProfileCannotWrite(std::io::Error),

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
    let mut path_dst = ensure_storage()?;
    let profile = Profile::new(name);
    path_dst.push(profile.filename());

    if !clobber && path_dst.exists() {
        Err(Error::ProfileExists(profile))
    } else {
        match path_file_credentials() {
            Some(path_src) => match std::fs::copy(path_src, &path_dst) {
                Ok(..) => Ok(Success::Saved(profile)),
                Err(e) => Err(Error::CredentialsCannotWrite(e)),
            }
            None => Err(Error::CredentialsNoPath),
        }
    }
}


pub fn profile_load(name: String) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;

    todo!()
}


pub fn profile_remove(name: Vec<String>, confirm: bool) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;

    todo!()
}
