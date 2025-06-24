use dirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections, env, error, fs, path, str};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub opn: Vec<OpnEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpnEntry {
    pub pattern: String,
    pub run: String,
    pub description: Option<String>,
    pub command: Option<collections::BTreeMap<String, CommandEntry>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandEntry {
    pub run: String,
    pub description: Option<String>,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        let mut seen_patterns = std::collections::HashSet::new();

        for entry in &self.opn {
            if !seen_patterns.insert(&entry.pattern) {
                return Err(format!("Duplicate pattern found: {}", entry.pattern));
            }
        }

        Ok(())
    }
}

pub fn load_config(path: Option<String>) -> Result<Config, Box<dyn error::Error>> {
    let home_dir = dirs::home_dir().unwrap().join(".opn");
    let mut dir = env::current_dir()?;
    let config_path = path.unwrap_or(home_dir.to_str().unwrap().to_string());

    loop {
        let candidate = dir.join(config_path.clone());

        if candidate.exists() {
            let contents = fs::read_to_string(candidate)?;
            let config: Config = serde_yaml::from_str(&contents)?;

            // Validate the config
            config.validate()?;

            return Ok(config);
        }

        if !dir.pop() {
            break;
        }
    }

    Err("No .opn found in current or parent directories.".into())
}

pub struct ResolvedCommand {
    pub run: String,
    pub key: String,
    pub context: String,
}

/// Check if a pattern matches a string using regex matching
fn matches_regex(pattern: &str, text: &str) -> bool {
    match Regex::new(pattern) {
        Ok(regex) => regex.is_match(text),
        Err(_) => false,
    }
}

pub fn resolve_option(
    context: &str,
    command: &Option<String>,
    config: &Config,
) -> Option<ResolvedCommand> {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(context);

    let path_exists = file_path.exists();

    if !path_exists {
        println!("{} does not exist", file_path.display());
        return None;
    }

    for opn_entry in &config.opn {
        // verbose
        println!("context: {}", context);
        println!("opn_entry: {}", opn_entry.pattern);

        let match_context = if file_path.is_dir() {
            let file_path_str = file_path.to_str().unwrap().to_string();

            let folder_context = path::Path::new(&file_path_str).join(opn_entry.pattern.as_str());

            let possible_file = path::Path::new(&folder_context);
            println!("possible_file: {}", possible_file.display());

            if possible_file.exists() {
                possible_file.to_str().unwrap().to_string()
            } else {
                path::Path::new(&file_path_str).to_str().unwrap().to_string()
            }
        } else {
            context.to_string()
        };

        println!("match_context: {}", match_context);

        let opn_context_match = opn_entry.pattern.to_string() + "$";

        if matches_regex(opn_context_match.as_str(), match_context.as_str()) {
            if let Some(context_command) = command {
                if let Some(commands) = &opn_entry.command {
                    for (opn_command, command_entry) in commands {
                        if opn_command == context_command {
                            return Some(ResolvedCommand {
                                run: command_entry.run.clone(),
                                key: opn_entry.pattern.clone(),
                                context: context.to_string(),
                            });
                        }
                    }

                    println!("Commands for {}:", opn_entry.pattern);

                    if let Some(commands) = &opn_entry.command {
                        for (command, command_entry) in commands {
                            println!(
                                "{}: {}",
                                command,
                                command_entry
                                    .description
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                            );
                        }
                    }
                }

                println!("{} not found in {}", context_command, opn_entry.pattern);

                return None;
            } else {
                return Some(ResolvedCommand {
                    run: opn_entry.run.clone(),
                    key: opn_entry.pattern.clone(),
                    context: context.to_string(),
                });
            }
        }
    }

    println!("{} pattern not found in config.", context);

    None
}

// MVP
// TODO: Add option to upsert config with/without context/command using -a

// Future
// TODO: Add a function to add a option to the config
// TODO: Add a function to edit a option from the config
// TODO: Add a function to remove a option from the config
// TODO: Add shell hook to show config options
// TODO: Add bash command to keep in same process
