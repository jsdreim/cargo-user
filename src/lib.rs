pub mod dirs;
pub mod errors;
pub mod profile;

use std::{fs::create_dir_all, path::PathBuf};
use dirs::{path_dir_storage, path_file_credentials};
pub use errors::{Error, ErrorStorage, Success};
pub use profile::Profile;


pub fn ensure_storage() -> Result<PathBuf, ErrorStorage> {
    let path_dir = path_dir_storage().ok_or(ErrorStorage::NoPath)?;

    if path_dir.exists() {
        if path_dir.is_dir() {
            Ok(path_dir)
        } else {
            Err(ErrorStorage::NotDir)
        }
    } else {
        match create_dir_all(&path_dir) {
            Ok(()) => Ok(path_dir),
            Err(e) => Err(ErrorStorage::CannotCreate(e)),
        }
    }
}


pub fn profile_clear() -> Result<Success, Error> {
    match path_file_credentials() {
        Some(path_cred) => match std::fs::remove_file(path_cred) {
            Ok(()) => Ok(Success::Cleared),
            Err(e) => Err(Error::CredentialsCannotRemove(e)),
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
            Some(path) if !path.is_file() => Err(Error::CredentialsNotFound),

            Some(path_src) => match std::fs::copy(path_src, &path_dst) {
                Ok(..) => Ok(Success::Saved(profile)),
                Err(e) => Err(Error::CannotSave(e)),
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
            Err(e) => Err(Error::CannotLoad(e)),
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
