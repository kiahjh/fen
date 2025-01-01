use crate::codegen::write_to_file;
use parser::codegen::GenCode;

pub fn gen_swift_client(
    path: &str,
    dev_endpoint: &str,
    prod_endpoint: &str,
    routes: Vec<&parser::ast::FileNode>,
) -> Result<(), std::io::Error> {
    let api_client_text = include_str!("templates/Api.swift");

    let comment = format!("// Created by Fen v{} at {} on {}\n// Do not manually modify this file as it is automatically generated", crate::VERSION, chrono::Local::now().format("%H:%M:%S"), chrono::Local::now().format("%Y-%m-%d"));

    write_to_file(
        path,
        "Api.swift",
        &api_client_text.replace("{{API_ENDPOINT}}", dev_endpoint),
        &comment,
    )?;

    for route in routes {
        write_to_file(
            path,
            &format!("{}.swift", route.name),
            &route.swift_client_code(),
            &comment,
        )?;
    }

    Ok(())
}
