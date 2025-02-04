use colored::Colorize;

use crate::{
    codegen::{client::swift::gen_swift_client, server::rust::gen_rust_server},
    config::{
        find_config_dir, get_config_file_contents,
        verification::{get_config_info, Language},
    },
    print::{info, success},
    routes::parse,
    VERSION,
};

pub fn run() -> Result<(), std::io::Error> {
    println!(
        "Running {} {}...",
        "Fen".yellow().bold(),
        VERSION.yellow().bold()
    );

    let config_dir = find_config_dir()?;
    let file_contents = get_config_file_contents(&config_dir)?;
    let config_info = get_config_info(&file_contents)?;
    let routes = parse(config_dir.to_str().unwrap())?;

    info(&format!(
        "Generating client-side code ({})...",
        config_info
            .client
            .outputs
            .iter()
            .map(|o| match o.language {
                Language::Rust => "Rust",
                Language::Swift => "Swift",
            })
            .collect::<Vec<&str>>()
            .join(", ")
    ));
    for output in config_info.client.outputs {
        if output.language == Language::Swift {
            gen_swift_client(
                &output.path,
                &config_info.client.endpoint_dev,
                &config_info.client.endpoint_prod,
                routes.iter().collect(),
            )?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported language: {:?}", output.language),
            ));
        }
    }
    success("Client-side code generated successfully!");

    info(&format!(
        "Generating server-side code ({})...",
        match &config_info.server.output.language {
            Language::Rust => "Rust",
            Language::Swift => "Swift",
        }
    ));
    if config_info.server.output.language == Language::Rust {
        gen_rust_server(&config_info.server.output.path, routes.iter().collect())?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Unsupported language: {:?}",
                config_info.server.output.language
            ),
        ));
    }
    success("Server-side code generated successfully!\n");

    success("That's it! Enjoy your typesafe API! 😊");

    println!();
    Ok(())
}
