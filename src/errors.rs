use crate::Profile;


#[derive(Debug)]
pub enum ErrorStorage {
    NoPath,
    NotDir,
    CannotCreate(std::io::Error),
}


#[derive(Debug)]
pub enum Error {
    CannotLoad(std::io::Error),
    CannotSave(std::io::Error),

    CredentialsNoPath,
    CredentialsNotFound,
    CredentialsCannotRemove(std::io::Error),

    ProfileExists(Profile),
    ProfileNotFound(Profile),
    ProfileCannotRemove(std::io::Error),

    Storage(ErrorStorage),
}

impl From<ErrorStorage> for Error {
    fn from(err: ErrorStorage) -> Self { Self::Storage(err) }
}


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
