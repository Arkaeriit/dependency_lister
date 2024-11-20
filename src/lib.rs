mod file_navigation;

use std::collections::HashSet;

/* ---------------------------- Exported function --------------------------- */


/// From a directory, read all d files and read all the dependencies listed in
/// them.
pub fn get_all_dependencies_from_dir(dir_path: &str) -> Result<HashSet<String>, DependencyListerError> {
    file_navigation::get_all_dependencies_from_dir(dir_path)
}

/* --------------------------------- Errors --------------------------------- */

#[derive(Debug, PartialEq)]
pub enum DependencyListerError {
    UnsuportedFileEncoding(Vec<u8>),
    UnableToRead(String),
    NotALink(String),
    UnsuportedFileName(String),
    DirectoryReadingError(String),
}

