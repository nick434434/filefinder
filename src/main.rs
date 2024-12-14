use std::collections::HashMap;

mod command;
mod validation;
mod crawler;

fn main() {
    let filefinder_cmd = command::filefinder_cmd();

    let args_map: HashMap<String, String> = command::extract_args(&filefinder_cmd);

    println!("Args: {:?}", args_map);

    if args_map.contains_key("name") {
        let directory = args_map.get("directory").unwrap();
        let query = args_map.get("query").unwrap();

        let result = crawler::search_by_filename(directory, query);

        match result {
            Some(file) => println!("File found: {}", file),
            None => println!("File not found")
        }
    }
}
