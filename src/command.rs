use std::collections::HashMap;
use clap::{Arg, ArgAction, ArgMatches};
use clap::Command;
use crate::validation::validate;

pub fn filefinder_cmd() -> Command {
    let command: Command = Command::new("filefinder")
        .arg(Arg::new("directory")
            .short('d')
            .required(false)
            .action(ArgAction::Set)
            .default_value("~"))
        .arg(Arg::new("name")
            .short('n')
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("recursive")
            .short('r')
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("extension")
            .short('e')
            .required(false)
            .action(ArgAction::Set))
        .arg(Arg::new("query")
            .required(true));

    command
}

pub fn get_argument_value(arg_matches: &ArgMatches, argument_name: &str) -> String {
    let raw = arg_matches.get_raw(argument_name);
    if let Some(mut value) = raw {
        value.next().expect("Argument value is required").to_os_string().into_string().expect("Argument value must be a string")
    } else {
        "".to_string()
    }
}

pub fn extract_args(command: &Command) -> HashMap<String, String> {
    let matches = command.clone().get_matches();

    let mut args_map: HashMap<String, String> = HashMap::new();

    for argument in command.clone().get_arguments() {
        let arg_value = get_argument_value(&matches, argument.get_id().as_ref());

        validate(argument.get_id().to_string(), arg_value.to_string()).expect("Invalid argument value");

        if !arg_value.is_empty() {
            args_map.insert(argument.get_id().to_string(), arg_value.to_string());
        }
    }

    args_map
}
