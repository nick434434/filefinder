use std::collections::HashMap;
use crate::command::extract_args;

mod command;
mod validation;
mod crawler;

fn main() {
    let filefinder_cmd = command::filefinder_cmd();

    let args_map: HashMap<String, String> = extract_args(&filefinder_cmd);

    println!("Args: {:?}", args_map);
}
