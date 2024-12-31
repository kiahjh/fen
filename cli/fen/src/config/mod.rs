use crate::print::{info, success};
use colored::Colorize;

pub mod verification;

pub fn get_config_file_contents(config_dir: &std::path::Path) -> Result<String, std::io::Error> {
    let config_file = config_dir.join("config.toml");

    info("Reading config file...");
    let contents = std::fs::read_to_string(&config_file)?;

    success(&format!(
        "Read config file: {}",
        config_file.display().to_string().bold().green()
    ));

    Ok(contents)
}

pub fn find_config_dir() -> Result<std::path::PathBuf, std::io::Error> {
    info("Looking for config directory...");
    let dirs = std::fs::read_dir(".")?;
    for dir in dirs {
        let dir = dir?;
        if dir.file_name() == "fen" {
            success(&format!(
                "Found config directory: {}",
                dir.path().display().to_string().bold().green()
            ));
            return Ok(dir.path());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find config directory",
    ))
}
