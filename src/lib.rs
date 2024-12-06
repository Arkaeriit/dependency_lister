mod file_navigation;

use std::collections::HashSet;
use std::fmt;

/* ---------------------------- Exported function --------------------------- */


/// From a directory, read all d files and read all the dependencies listed in
/// them.
pub fn get_all_dependencies_from_dir(dir_path: &str) -> Result<HashSet<String>, DependencyListerError> {
    file_navigation::get_all_dependencies_from_dir(dir_path)
}

/* --------------------------------- Errors --------------------------------- */

/// Used to represent errors that can be raised by the dependency listing.
#[derive(Debug, PartialEq)]
pub enum DependencyListerError {
    /// A file name encoding is not supported.
    UnsuportedFileEncoding(Vec<u8>),

    /// Unable to read a file.
    UnableToRead(String),

    /// A file is not a link while we expected it to be.
    NotALink(String),

    /// There is an error manipulating a file name.
    UnsuportedFileName(String),

    /// We can't read a directory.
    DirectoryReadingError(String),
}

use DependencyListerError::*;

impl fmt::Display for DependencyListerError {
    /// From a `DependencyListerError`, makes an error message that could even be
    /// shown to the final user.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnsuportedFileEncoding(x) => write!(f, "Unsuported file name encoding. Binary file name: {x:?}"),
            UnableToRead(x) => write!(f, "Unable to read the file {x}"),
            NotALink(x) => write!(f, "Expected {x} to be a link but it is not."),
            UnsuportedFileName(x) => write!(f, "Unsuported file name {x}"),
            DirectoryReadingError(x) => write!(f, "Error while reading directory {x}"),
        }
    }
}

