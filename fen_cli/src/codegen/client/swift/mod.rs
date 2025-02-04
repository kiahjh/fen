use crate::codegen::write_to_file;
use fen_parser::codegen::{Context, GenCode};

pub fn gen_swift_client(
    path: &str,
    dev_endpoint: &str,
    prod_endpoint: &str,
    routes: Vec<&fen_parser::ast::FileNode>,
) -> Result<(), std::io::Error> {
    let api_client_text = include_str!("templates/Api.swift");

    write_to_file(path, "Api.swift", api_client_text)?;

    for route in routes {
        write_to_file(
            path,
            &format!("{}.swift", route.name),
            &route.swift_client_code(&Context {
                override_name: None,
            }),
        )?;
    }

    Ok(())
}
