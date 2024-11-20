use crate::DependencyListerError::*;
use crate::DependencyListerError;
use std::collections::HashSet;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

/* ----------------------------- File navigation ---------------------------- */

/// List all the files from the `from` dir recursively, and if their path is
/// sufixed by `filename_suffix`, add them to the returned set.
fn find_files(from: &str, filename_suffix: &str) -> Result<HashSet<String>, DependencyListerError> {
    let mut ret = HashSet::<String>::new();
    fn find_files_and_fill_set(from: &str, filename_suffix: &str, set: &mut HashSet<String>) -> Result<(), DependencyListerError>{
        let matching = filename_suffix.as_bytes();
        let read_dir = if let Ok(dir) = fs::read_dir(from) {
            dir
        } else {
            return Err(DirectoryReadingError(from.to_string()));
        };
        for path_entry in read_dir {
            let path = if let Ok(p) = path_entry {
                p.path()
            } else {
                return Err(DirectoryReadingError(from.to_string()));
            };
            if path.is_dir() {
                find_files_and_fill_set(path_to_str(&path)?, filename_suffix, set)?;
            }
            let bytes = path_to_str(&path)?.as_bytes();
            if bytes.len() < matching.len() {
                continue;
            }
            let byte_offset = bytes.len() - matching.len();
            let mut to_insert = true;
            for i in 0..matching.len() {
                if bytes[i+byte_offset] != matching[i] {
                    to_insert = false;
                    continue;
                }
            }
            if to_insert {
                set.insert(path_to_str(&path)?.to_string());
            }
        }
        Ok(())
    }
    find_files_and_fill_set(from, filename_suffix, &mut ret)?;
    Ok(ret)
}

/// From a path to a file, if the file is a link, return it's target.
/// It it's not, return itself.
fn follow_links_if_needed(path: &str) -> Result<String, DependencyListerError> {
    let mut ret = path.to_string();
    let mut metadata = symlink_metadata(path)?;
    while metadata.is_symlink() {
        let target_path = read_link(&ret)?;
        let target = path_to_str(&target_path)?;
        if target_path.is_absolute() {
            ret = target.to_string();
        } else {
            let dir = if let Some(maybe_dir) = Path::new(&ret).parent() {
                path_to_str(maybe_dir)?
            } else {
                ""
            };
            ret = format!("{dir}/{target}");
        }
        metadata = symlink_metadata(&ret)?;
    }
    Ok(ret)
}

/// Read the input .d file and put all listed files in the input set.
/// Follow symlinks if needed.
fn read_d_file(file: &str, set: &mut HashSet<String>) -> Result<(), DependencyListerError> {
    let mut first_line = true;
    let txt = if let Ok(txt) = fs::read_to_string(file) {
        txt
    } else {
        return Err(UnableToRead(file.to_string()));
    };
    for line in txt.split('\n') {
        if first_line {
            first_line = false;
            continue;
        }
        let bytes = line.as_bytes();
        if bytes.len() < 4 {
            continue;
        }
        let cut_bytes = if bytes[bytes.len()-1] == b'\\' {
            &bytes[1..bytes.len()-2]
        } else {
            &bytes[1..bytes.len()]
        };

        let path_from_d_file = if let Ok(converted) = std::str::from_utf8(cut_bytes) {
            converted
        } else {
            return Err(UnsuportedFileEncoding(cut_bytes.to_vec()));
        };
        let target = follow_links_if_needed(path_from_d_file)?;
        set.insert(target);
    }
    Ok(())
}

/// From a directory, read all d files and read all the dependencies listed in
/// them.
pub fn get_all_dependencies_from_dir(dir_path: &str) -> Result<HashSet<String>, DependencyListerError> {
    let all_d = find_files(dir_path, ".d")?;
    let mut ret = HashSet::new();
    for d_file in all_d {
        read_d_file(&d_file, &mut ret)?;
    }
    Ok(ret)
}

/* ------------------------------ Error helpers ----------------------------- */

fn symlink_metadata(path: &str) -> Result<fs::Metadata, DependencyListerError> {
    match fs::symlink_metadata(path) {
        Ok(x) => Ok(x),
        Err(_) => Err(NotALink(path.to_string())),
    }
}

fn path_to_str(path: &Path) -> Result<&str, DependencyListerError> {
    match path.to_str() {
        Some(x) => Ok(x),
        None => Err(UnsuportedFileName(format!("{}", path.display()))),
    }
}

fn read_link(path: &str) -> Result<PathBuf, DependencyListerError> {
    match fs::read_link(path) {
        Ok(x) => Ok(x),
        Err(_) => Err(UnableToRead(path.to_string())),
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
fn test_find_file() {
    assert_eq!(find_files("./test", ".rs"), Ok(HashSet::from([
                                                      "./test/src/main.rs".to_string(),
                                                      "./test/src/lib.rs".to_string(),
                                                      "./test/test.rs".to_string(),
    ])));
}

#[test]
fn test_follow_links_if_needed() {
    assert_eq!(follow_links_if_needed("./test/source"), Ok("./test/source".to_string()));
    assert_eq!(follow_links_if_needed("./test/link1"), Ok("./test/source".to_string()));
    assert_eq!(follow_links_if_needed("./test/link2"), Ok("./test/source".to_string()));
    assert_eq!(follow_links_if_needed("./test/readme.md"), Ok("./test/readme.md".to_string()));
}

#[test]
fn test_read_d_file() {
    let mut set = HashSet::new();
    assert_eq!(read_d_file("./test/test.d", &mut set), Ok(()));
    assert_eq!(set, HashSet::from([
                               "./test/src/main.rs".to_string(),
                               "./test/source".to_string(),
    ]));
}

#[test]
fn test_get_all_dependencies_from_dir() {
    assert_eq!(get_all_dependencies_from_dir("./test"), Ok(HashSet::from([
                               "./test/src/main.rs".to_string(),
                               "./test/source".to_string(),
    ])));
}

