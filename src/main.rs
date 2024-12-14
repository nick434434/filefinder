use std::collections::HashMap;

mod command;
mod validation;
mod crawler;

fn main() {
    let filefinder_cmd = command::filefinder_cmd();

    let args_map: HashMap<String, String> = command::extract_args(&filefinder_cmd);

    println!("Args: {:?}", args_map);

    let directory = args_map.get("directory").unwrap();
    let query = args_map.get("query").unwrap();
    let extension = if args_map.contains_key("extension") {args_map.get("extension")} else {None};
    let is_recursive = command::arg_to_bool(args_map.get("recursive").unwrap());
    let is_name = command::arg_to_bool(args_map.get("name").unwrap());

    let result: Option<String> = if is_name  {
        crawler::search_by_filename(directory, query, is_recursive, extension)
    } else {
        crawler::search_by_contents(directory, query, is_recursive, extension)
    };

    match result {
        Some(file) => println!("File found: {}", file),
        None => println!("File not found")
    }
}
