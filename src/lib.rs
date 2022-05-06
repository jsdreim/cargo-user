pub mod dirs;
pub mod errors;
pub mod profile;

use std::{fs::create_dir_all, io::Read, path::{Path, PathBuf}};
use dirs::{path_dir_storage, path_file_credentials};
pub use errors::{Error, ErrorStorage, Success};
pub use profile::Profile;


fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut file = std::fs::File::open(path)?;
    file.read_to_end(&mut data)?;

    Ok(data)
}


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


pub fn profile_list() -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;
    let mut profiles = Vec::new();

    if let Ok(dir) = dir_profile.read_dir() {
        for entry in dir {
            if let Ok(sub) = entry {
                if let Some(profile) = Profile::from_path(sub.path()) {
                    profiles.push(profile);
                }
            }
        }
    }

    Ok(Success::List(profiles))
}


pub fn profile_current() -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;
    let mut current = Vec::new();

    match path_file_credentials() {
        None => return Err(Error::CredentialsNoPath),
        Some(path) if !path.is_file() => return Err(Error::CredentialsNotFound),

        Some(path_src) => if let Ok(dir) = dir_profile.read_dir() {
            let creds = read(path_src).map_err(Error::CredentialsCannotRead)?;

            for entry in dir.filter_map(|e| e.ok()) {
                let path = entry.path();

                if let Some(profile) = Profile::from_path(&path) {
                    match read(&path) {
                        Ok(data) => if data == creds {
                            current.push(profile);
                        }
                        Err(err) => return Err(Error::ProfileCannotRead(profile, err)),
                    }
                }
            }
        }
    }

    Ok(Success::Current(current))
}


pub fn profile_find(name_opt: Option<String>) -> Result<Success, Error> {
    let found: Result<PathBuf, ErrorStorage> = match name_opt {
        Some(name) => Profile::new(name).path().ok_or(ErrorStorage::NoPath),
        None => ensure_storage(),
    };

    Ok(Success::Found(found?))
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

            Some(path_src) => match std::fs::copy(path_src, path_dst) {
                Ok(..) => Ok(Success::Saved(profile)),
                Err(e) => Err(Error::CannotSave(profile, e)),
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
        Some(path_dst) => match std::fs::copy(path_src, path_dst) {
            Ok(..) => Ok(Success::Loaded(profile)),
            Err(e) => Err(Error::CannotLoad(profile, e)),
        }
        None => Err(Error::CredentialsNoPath),
    }
}


pub fn profile_rename(
    name_old: String,
    name_new: String,
    clobber: bool,
) -> Result<Success, Error> {
    let mut path_src = ensure_storage()?;
    let mut path_dst = path_src.clone();

    let old = Profile::new(name_old);
    path_src.push(old.filename());

    let new = Profile::new(name_new);
    path_dst.push(new.filename());

    if !clobber && path_dst.exists() {
        Err(Error::ProfileExists(new))
    } else {
        match std::fs::rename(path_src, path_dst) {
            Ok(..) => Ok(Success::Renamed(old, new)),
            Err(e) => Err(Error::ProfileCannotRename(old, new, e)),
        }
    }
}


pub fn profile_remove(names: Vec<String>) -> Result<Success, Error> {
    let dir_profile = ensure_storage()?;
    let mut vec_del = Vec::with_capacity(names.len());
    let mut vec_err = Vec::with_capacity(names.len());

    for name in names {
        let profile = Profile::new(name);

        let mut path = dir_profile.clone();
        path.push(profile.filename());

        if path.is_file() {
            match std::fs::remove_file(&path) {
                Ok(()) => vec_del.push(profile),
                Err(e) => vec_err.push(Error::ProfileCannotRemove(profile, e)),
            }
        } else {
            vec_err.push(Error::ProfileNotFound(profile));
        }
    }

    Ok(Success::Removed { removed: vec_del, errors: vec_err })
}
