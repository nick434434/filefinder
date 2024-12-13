use clap::Command;
use std::collections::HashMap;

mod command;


fn main() {
    let filefinder_cmd: Command = command::filefinder_cmd();

    let matches = filefinder_cmd.clone().get_matches();
    
    let mut args_map: HashMap<&str, &str> = HashMap::new();

    for argument in filefinder_cmd.get_arguments() {
        args_map.insert(argument.get_id().as_ref(), command::get_argument_value(&matches, argument.get_id().as_ref()));
    }
    
    println!("Args: {:?}", args_map);
}
