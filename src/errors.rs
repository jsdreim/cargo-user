use crate::Profile;


#[derive(Debug)]
pub enum ErrorStorage {
    NoPath,
    NotDir,
    CannotCreate(std::io::Error),
}


#[derive(Debug)]
pub enum Error {
    CannotLoad(Profile, std::io::Error),
    CannotSave(Profile, std::io::Error),

    CredentialsNoPath,
    CredentialsNotFound,
    CredentialsCannotRemove(std::io::Error),

    ProfileExists(Profile),
    ProfileNotFound(Profile),
    ProfileCannotRemove(Profile, std::io::Error),

    Storage(ErrorStorage),
}

impl From<ErrorStorage> for Error {
    fn from(err: ErrorStorage) -> Self { Self::Storage(err) }
}


#[derive(Debug)]
pub enum Success {
    List(Vec<Profile>),
    Saved(Profile),
    Loaded(Profile),
    Cleared,
    Removed {
        removed: Vec<Profile>,
        errors: Vec<Error>,
    },
}
