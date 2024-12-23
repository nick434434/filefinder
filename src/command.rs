use dirs::home_dir;
use clap::{Arg, ArgAction, ArgMatches};
use clap::Command;
use crate::validation;
use std::collections::HashMap;

pub fn filefinder_cmd() -> Command {
    let command: Command = Command::new("filefinder")
        .about("Search for files by name or contents")
        .arg(Arg::new("directory")
            .short('d')
            .required(false)
            .action(ArgAction::Set)
            .default_value("~")
            .help("Directory to search in"))
        .arg(Arg::new("name")
            .short('n')
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Search by file name")
            .long_help("If set, the search is done by file name, otherwise - by file contents"))
        .arg(Arg::new("recursive")
            .short('r')
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Crawl directory recursively")
            .long_help("If set, the crawl is done recursively, otherwise - only in the specified folder"))
        .arg(Arg::new("extension")
            .short('e')
            .required(false)
            .action(ArgAction::Set)
            .help("Extension of the files to search for")
            .long_help("If set, only the files with such extension are considered, otherwise - all files are eligible"))
        .arg(Arg::new("multiple")
            .short('m')
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Return all matches")
            .long_help("If set, all matches are returned, otherwise - only the first one"))
        .arg(Arg::new("query")
            .required(true)
            .help("The search string")
            .long_help("The search string which must be a valid regex pattern including a simple word/phrase"));

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
        let mut arg_value = get_argument_value(&matches, argument.get_id().as_ref());

        if argument.get_id() == "directory" && arg_value.starts_with("~") {
            arg_value = arg_value.to_string().replace(
                "~",
                home_dir().expect("Home directory not found").to_str().expect("Home directory is not a valid string")
            );
        }

        validation::validate(argument.get_id().to_string(), arg_value.to_string()).expect("Invalid argument value");

        if !arg_value.is_empty() {
            args_map.insert(argument.get_id().to_string(), arg_value.to_string());
        }
    }

    args_map
}

pub fn arg_to_bool(arg: &str) -> bool {
    arg.to_lowercase() == "true"
}
