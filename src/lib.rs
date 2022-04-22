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
    Removed {
        removed: Vec<Profile>,
        errors: Vec<Error>,
    },
}


#[derive(Debug)]
pub enum Error {
    CredentialsNoPath,
    CredentialsNotFound,
    CredentialsCannotRead(std::io::Error),
    CredentialsCannotWrite(std::io::Error),
    CredentialsCannotClear(std::io::Error),

    ProfileExists(Profile),
    ProfileNoPath(Profile),
    ProfileNotFound(Profile),
    ProfileCannotRead(std::io::Error),
    ProfileCannotWrite(std::io::Error),
    ProfileCannotRemove(std::io::Error),

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
    match path_file_credentials() {
        Some(path_cred) => match std::fs::remove_file(path_cred) {
            Ok(()) => Ok(Success::Cleared),
            Err(e) => Err(Error::CredentialsCannotClear(e)),
        }
        None => Err(Error::CredentialsNoPath),
    }
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
                Err(e) => Err(Error::ProfileCannotWrite(e)),
            }
            None => Err(Error::CredentialsNoPath),
        }
    }
}


pub fn profile_load(name: String) -> Result<Success, Error> {
    let mut path_src = ensure_storage()?;
    let profile = Profile::new(name);
    path_src.push(profile.filename());

    match path_file_credentials() {
        Some(path_dst) => match std::fs::copy(path_src, &path_dst) {
            Ok(..) => Ok(Success::Loaded(profile)),
            Err(e) => Err(Error::CredentialsCannotWrite(e)),
        }
        None => Err(Error::CredentialsNoPath),
    }
}


pub fn profile_remove(names: Vec<String>, confirm: bool) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;
    let mut vec_del = Vec::with_capacity(names.len());
    let mut vec_err = Vec::with_capacity(names.len());

    if confirm {
        //  TODO
    }

    for name in names {
        let profile = Profile::new(name);

        let mut path = dir_profile.clone();
        path.push(profile.filename());

        if path.is_file() {
            match std::fs::remove_file(&path) {
                Ok(()) => vec_del.push(profile),
                Err(e) => vec_err.push(Error::ProfileCannotRemove(e)),
            }
        } else {
            vec_err.push(Error::ProfileNotFound(profile));
        }
    }

    Ok(Success::Removed { removed: vec_del, errors: vec_err })
}
