use std::fs::File;
use std::io::prelude::*;

pub mod client;
pub mod server;

pub fn write_to_file(dir: &str, file_name: &str, text: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("{dir}/{file_name}"))?;
    let content = format!("{}\n\n{}", comment(), text);
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn comment() -> String {
    format!("// Created by Fen v{} at {} on {}\n// Do not manually modify this file as it is automatically generated", crate::VERSION, chrono::Local::now().format("%H:%M:%S"), chrono::Local::now().format("%Y-%m-%d"))
}
