use std::env::args;
use std::process::exit;
use std::collections::HashSet;
use dependency_lister::get_all_dependencies_from_dir;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        help();
        let exit_code = if args.len() == 1 {
            0
        } else {
            1
        };
        exit(exit_code);
    }

    exit(match get_all_dependencies_from_dir(&args[1]) {
        Ok(dependancies) => {
            display_set(&dependancies);
            0
        },
        Err(err) => {
            eprintln!("{err}");
            2
        },
    });
}

/// Print an help message.
fn help() {
    println!("Print all the dependencies noted in .d files from a directory.");
    println!("Usage:");
    println!("	dependency-lister <directory>");
}

/// Print all the elements in a HashSet one on each line.
fn display_set<T: std::fmt::Display>(set: &HashSet<T>) {
    for element in set {
        println!("{element}");
    }
}

