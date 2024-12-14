use regex::Regex;
use std::fs::read_to_string;
use std::iter::Filter;
use std::result::Result;
use walkdir::{DirEntry, Error, IntoIter, WalkDir};

fn crawl_directory(directory: &str, recursive: bool) -> Filter<IntoIter, fn(&Result<DirEntry, Error>) -> bool> {
    let crawl = if recursive {
        WalkDir::new(directory)
    } else {
        WalkDir::new(directory).max_depth(1)
    };

    crawl.into_iter().filter(|e: &Result<DirEntry, Error>| e.is_ok() && !e.as_ref().unwrap().path().is_dir())
}

fn is_correct_extension(entry: &DirEntry, extension: Option<&String>) -> bool {
    if extension.is_none() {
        return true;
    }

    let ext = entry.path().extension();
    if ext.is_none() {
        // We could theoretically search for a file without extension 
        return extension.unwrap().is_empty();
    }

    ext.unwrap() == extension.unwrap().as_str()
}

pub fn search_by_filename(directory: &str, query: &str, recursive: bool, extension: Option<&String>) -> Option<String> {
    let re = Regex::new(query).unwrap();

    for entry in crawl_directory(directory, recursive).filter_map(|e| e.ok()) {
        if is_correct_extension(&entry, extension) && re.is_match(entry.path().file_name().unwrap().to_str().unwrap()) {
            return Some(entry.path().to_str().unwrap().to_string());
        }
    }

    None
}

pub fn search_by_contents(directory: &str, query: &str, recursive: bool, extension: Option<&String>) -> Option<String> {
    let re = Regex::new(query).unwrap();

    for entry in crawl_directory(directory, recursive).filter_map(|e| e.ok()) {
        let file_contents = read_to_string(entry.path());
        if file_contents.is_err() {
            continue
        }

        if is_correct_extension(&entry, extension) && re.is_match(file_contents.unwrap().as_str()) {
            return Some(entry.path().to_str().unwrap().to_string());
        }
    }

    None
}
