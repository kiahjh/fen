use colored::Colorize;

use crate::{
    codegen::client::swift::gen_swift_client,
    config::{
        find_config_dir, get_config_file_contents,
        verification::{get_config_info, Language},
    },
    print::info,
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
            )?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported language: {:?}", output.language),
            ));
        }
    }

    println!();

    Ok(())
}
