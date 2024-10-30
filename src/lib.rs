use std::collections::HashSet;
use std::path::Path;
use std::fs;

/* ----------------------------- File navigation ---------------------------- */

/// List all the files from the `from` dir recursively, and if their path is
/// sufixed by `filename_suffix`, add them to the returned set.
fn find_files(from: &str, filename_suffix: &str) -> HashSet<String> {
    let mut ret = HashSet::<String>::new();
    fn find_files_and_fill_set(from: &str, filename_suffix: &str, set: &mut HashSet<String>) {
        let matching = filename_suffix.as_bytes();
        for path_entry in fs::read_dir(from).unwrap() {
            let path = path_entry.unwrap().path();
            if path.is_dir() {
                find_files_and_fill_set(path.to_str().unwrap(), filename_suffix, set);
            }
            let bytes = path.to_str().unwrap().as_bytes();
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
                set.insert(path.to_str().unwrap().to_string());
            }
        }
    }
    find_files_and_fill_set(from, filename_suffix, &mut ret);
    ret
}

/// From a path to a file, if the file is a link, return it's target.
/// It it's not, return itself.
fn follow_links_if_needed(path: &str) -> String {
    let mut ret = path.to_string();
    let mut metadata = fs::symlink_metadata(path).unwrap();
    while metadata.is_symlink() {
        let target_path = fs::read_link(&ret).unwrap();
        let target = target_path.to_str().unwrap();
        if target_path.is_absolute() {
            ret = target.to_string();
        } else {
            let dir = if let Some(maybe_dir) = Path::new(&ret).parent() {
                maybe_dir.to_str().unwrap()
            } else {
                ""
            };
            ret = format!("{dir}/{target}");
        }
        metadata = fs::symlink_metadata(&ret).unwrap();
    }
    ret
}

/// Read the input .d file and put all listed files in the input set.
/// Follow symlinks if needed.
fn read_d_file(file: &str, set: &mut HashSet<String>) {
    let mut first_line = true;
    for line in fs::read_to_string(file).unwrap().split('\n') {
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

        let path_from_d_file = std::str::from_utf8(cut_bytes).unwrap();
        let target = follow_links_if_needed(path_from_d_file);
        set.insert(target);
    }
}

/// From a directory, read all d files and read all the dependencies listed in
/// them.
fn get_all_dependencies_from_dir(dir_path: &str) -> HashSet<String> {
    let all_d = find_files(dir_path, ".d");
    let mut ret = HashSet::new();
    for d_file in all_d {
        read_d_file(&d_file, &mut ret);
    }
    ret
}

/* --------------------------------- Testing -------------------------------- */

#[test]
fn test_find_file() {
    assert_eq!(find_files("./test", ".rs"), HashSet::from([
                                                      "./test/src/main.rs".to_string(),
                                                      "./test/src/lib.rs".to_string(),
                                                      "./test/test.rs".to_string(),
    ]));
}

#[test]
fn test_follow_links_if_needed() {
    assert_eq!(follow_links_if_needed("./test/source"), "./test/source".to_string());
    assert_eq!(follow_links_if_needed("./test/link1"), "./test/source".to_string());
    assert_eq!(follow_links_if_needed("./test/link2"), "./test/source".to_string());
    assert_eq!(follow_links_if_needed("./test/readme.md"), "./test/readme.md".to_string());
}

#[test]
fn test_read_d_file() {
    let mut set = HashSet::new();
    read_d_file("./test/test.d", &mut set);
    assert_eq!(set, HashSet::from([
                               "./test/src/main.rs".to_string(),
                               "./test/source".to_string(),
    ]));
}

#[test]
fn test_get_all_dependencies_from_dir() {
    assert_eq!(get_all_dependencies_from_dir("./test"), HashSet::from([
                               "./test/src/main.rs".to_string(),
                               "./test/source".to_string(),
    ]));
}

