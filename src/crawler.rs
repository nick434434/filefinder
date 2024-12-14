use regex::Regex;
use walkdir::WalkDir;

pub fn search_by_filename(directory: &str, query: &str) -> Option<String> {
    let re = Regex::new(query).unwrap();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        if re.is_match(entry.path().file_name().unwrap().to_str().unwrap()) {
            return Some(entry.path().file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    None
}