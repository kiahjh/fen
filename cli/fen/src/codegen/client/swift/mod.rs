use crate::codegen::write_to_file;

pub fn gen_swift_client(
    path: &str,
    dev_endpoint: &str,
    prod_endpoint: &str,
) -> Result<(), std::io::Error> {
    let fetcher_text = include_str!("templates/Fetcher.swift");
    let api_client_text = include_str!("templates/ApiClient.swift");

    let comment = format!("// Created by Fen v{} at {} on {}\n// Do not manually modify this file as it is automatically generated", crate::VERSION, chrono::Local::now().format("%H:%M:%S"), chrono::Local::now().format("%Y-%m-%d"));

    write_to_file(path, "Fetcher.swift", fetcher_text, &comment)?;
    write_to_file(path, "ApiClient.swift", api_client_text, &comment)?;

    Ok(())
}
