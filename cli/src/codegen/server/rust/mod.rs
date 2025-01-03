use crate::codegen::write_to_file;
use parser::codegen::{name_transforms::pascal_to_snake, Context, GenCode};

pub fn gen_rust_server(
    path: &str,
    routes: Vec<&parser::ast::FileNode>,
) -> Result<(), std::io::Error> {
    let response_types_text = include_str!("templates/response.rs");

    let mods = routes
        .iter()
        .map(|route| {
            format!(
                "pub mod {name};",
                name = pascal_to_snake(&route.name).to_lowercase()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    write_to_file(path, "mod.rs", &format!("{mods}\n\n{response_types_text}"))?;

    for route in routes {
        write_to_file(
            path,
            &format!("{}.rs", pascal_to_snake(&route.name)),
            &route.rust_server_code(&Context {
                override_name: None,
                codeability: None,
            }),
        )?;
    }

    Ok(())
}
