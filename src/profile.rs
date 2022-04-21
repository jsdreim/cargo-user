use std::path::PathBuf;
use crate::dirs::path_dir_storage;


#[derive(Debug)]
pub struct Profile {
    name: String,
}

impl Profile {
    pub const fn new(name: String) -> Self { Self { name } }

    pub fn from_filename(filename: impl AsRef<str>) -> Option<Self> {
        filename.as_ref().strip_prefix("credentials-")
            .map(|name| Self::new(name.into()))
    }

    pub fn filename(&self) -> String {
        format!("credentials-{}", self.name)
    }

    pub fn path(&self) -> Option<PathBuf> {
        let mut path = path_dir_storage()?;
        path.push(self.filename());
        Some(path)
    }
}
