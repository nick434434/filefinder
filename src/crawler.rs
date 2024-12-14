use regex::Regex;
use std::fs::read_to_string;
use walkdir::WalkDir;

pub fn search_by_filename(directory: &str, query: &str, _extension: Option<&String>) -> Option<String> {
    let re = Regex::new(query).unwrap();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue
        }

        if re.is_match(entry.path().file_name().unwrap().to_str().unwrap()) {
            return Some(entry.path().to_str().unwrap().to_string());
        }
    }

    None
}

pub fn search_by_contents(directory: &str, query: &str, _extension: Option<&String>) -> Option<String> {
    let re = Regex::new(query).unwrap();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue
        }

        let file_contents = read_to_string(entry.path());
        if file_contents.is_err() {
            continue
        }

        if re.is_match(file_contents.unwrap().as_str()) {
            return Some(entry.path().to_str().unwrap().to_string());
        }
    }

    None
}
