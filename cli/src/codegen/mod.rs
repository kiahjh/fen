use std::fs::File;
use std::io::prelude::*;

pub mod client;

pub fn write_to_file(
    dir: &str,
    file_name: &str,
    text: &str,
    comment: &str,
) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("{dir}/{file_name}"))?;
    let content = format!("{comment}\n\n{text}");
    file.write_all(content.as_bytes())?;
    Ok(())
}
