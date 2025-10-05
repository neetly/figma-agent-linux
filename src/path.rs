use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum PathError {
    #[error("Failed to get home directory")]
    HomeNotDefined,
}

pub fn expand_home(path: impl AsRef<Path>) -> Result<PathBuf, PathError> {
    let path = path.as_ref();
    if let Ok(path_relative_to_home) = path.strip_prefix("~") {
        env::home_dir()
            .map(|home_directory| home_directory.join(path_relative_to_home))
            .ok_or(PathError::HomeNotDefined)
    } else {
        Ok(path.into())
    }
}
