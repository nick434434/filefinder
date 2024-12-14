use regex::Regex;
use std::path::Path;

fn validate_directory(directory: &str) -> Result<(), String> {
    if directory.is_empty() {
        return Err("Directory is not provided and no default is set".to_string());
    }

    if !Path::new(directory).exists() {
        return Err("Directory must exist".to_string());
    }

    Ok(())
}

fn validate_query(query: &str) -> Result<(), String> {
    if query.is_empty() {
        return Err("Query is required".to_string());
    }

    Regex::new(query)
        .map(|_| ())
        .map_err(|e| format!("Invalid query: {}", e))
}

fn validate_extension(extension: &str) -> Result<(), String> {
    if extension.is_empty() {
        return Ok(());
    }

    if extension.chars().all(|c| c.is_alphanumeric()) && !extension.is_empty() && extension.len() <= 4 {
        return Err("Extension must be between 1 and 4 alphanumeric characters".to_string());
    }

    Ok(())
}

pub fn validate(arg_key: String, arg_value: String) -> Result<(), String> {
    match arg_key.as_str() {
        "directory" => validate_directory(&arg_value),
        "extension" => validate_extension(&arg_value),
        "query" => validate_query(&arg_value),
        _ => Ok(())
    }
}
