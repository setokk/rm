use std::io;
use std::fs;
use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::exit;

// Constants
const NO_FILE_OR_DIR_SPECIFIED: &str = "";

fn manual() {
    println!("                -f, -F, --force
                ignore nonexistent files and arguments, never prompt

                -r, -R
                remove directories and their contents recursively

                -i
                prompt before every removal");
}

fn remove_file_prompt(path_name: &str, choice: &mut String) -> bool {
    println!("remove file \"{}\"? (y/n): ", path_name);

    io::stdin()
        .read_line(choice)
        .expect("Failed to read line");

    return choice.to_lowercase().eq("y\n");
}

fn recursive_path_delete(path: &str, mut choice: &mut String) {
    let entries: Vec<_> = fs::read_dir(path).unwrap().collect();
    for entry in &entries[..] {
        let entry: &DirEntry  = entry.as_ref().unwrap();
        let path_buffer: PathBuf = entry.path();
        let entry_path: &Path = path_buffer.as_path();
        let entry_str:  &str  = entry_path.to_str().unwrap();

        // Remove file prompt
        if entry_path.is_file() {
            if remove_file_prompt(entry_str, choice) {
                fs::remove_file(entry_path).unwrap();
            }
        }

        // Termination condition
        if (entry_path.is_file() && entries.len() == 1)
        || entries.is_empty() {
            return;
        }

        // Manage directory deletions
        if entry_path.is_dir() {
            println!("descent into {}? (y/n)", entry_str);
            io::stdin().read_line(&mut choice)
                .expect("Can't read line");

            if choice.to_lowercase().eq("y\n") {
                recursive_path_delete(entry_str, choice);
            }

            println!("remove directory {}?", entry_str);
            io::stdin().read_line(&mut choice)
                .expect("Can't read line");

            if choice.to_lowercase().eq("y\n") {
                fs::remove_dir(entry_path).
                    expect("Can't delete dir. Check permissions or check if dir is empty");
            }
        }
    }
}

fn main() {
    // Initialize flags
    let mut path: &str = NO_FILE_OR_DIR_SPECIFIED;
    let mut recursive: bool = false;
    let mut force:     bool = false;
    let mut prompt:    bool = false;

    // Get flags
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let arg: &str = args[i].as_str();
        if arg.to_lowercase().eq("-r") {
            recursive = true;
        } else if arg.to_lowercase().eq("-f")
               || arg.to_lowercase().eq("--force") {
            force = true;
        } else if arg.to_lowercase().eq("-rf") {
            recursive = true;
            force = true;
        } else if arg.to_lowercase().eq("-i") {
            prompt = true;
        } else if arg.to_lowercase().eq("-h")
               || arg.to_lowercase().eq("--help") {
            manual();
            exit(0);
        } else {
            path = arg;
        }
    }

    // Check if user provided path
    if path.eq(NO_FILE_OR_DIR_SPECIFIED) {
        eprintln!("No file/dir was specified\nExiting...");
        exit(1);
    }

    // Start deleting and shit
    let mut choice = String::new();
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                if force || remove_file_prompt(path, &mut choice) {
                    fs::remove_file(path)
                        .expect("Can't delete file. Check permissions");
                }
            } else if metadata.is_dir() {
                if force && recursive {
                    fs::remove_dir_all(path)
                        .expect("Can't remove dir/subdirs/subfiles. Check permissions");
                    exit(0);
                }

                if force {
                    eprintln!("Error in input. Expected file but got dir.\
                    Exiting...");
                }

                // only: recursive = true
                recursive_path_delete(path, &mut choice);
            }
        }
        Err(e) => {
            eprintln!("Error: {}\nExiting...", e);
            exit(1);
        }
    }
}