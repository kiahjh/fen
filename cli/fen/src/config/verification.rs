pub struct Info {
    pub client: ClientInfo,
    pub server: ServerInfo,
}

pub struct ClientInfo {
    pub outputs: Vec<Output>,
    pub endpoint_dev: String,
    pub endpoint_prod: String,
}

pub struct ServerInfo {
    pub output: Output,
}

pub struct Output {
    pub language: Language,
    pub path: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Language {
    Rust,
    Swift,
}

pub fn get_config_info(file_contents: &str) -> Result<Info, std::io::Error> {
    let table = file_contents.parse::<toml::Table>();

    if table.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid config file",
        ));
    }
    let table = table.unwrap();

    let client = verify_table(&table, "client")?;
    let server = verify_table(&table, "server")?;

    let client_output = verify_array(&client, "output")?;
    let server_output = verify_table(&server, "output")?;

    let mut client_outputs: Vec<Output> = Vec::new();
    for output in client_output {
        let output = output.as_table();
        if output.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Output configuration is not a table",
            ));
        }
        let output = output.unwrap();

        let language = verify_string(output, "language")?;
        let path = verify_string(output, "path")?;

        let language = match language.as_str() {
            "rust" => Language::Rust,
            "swift" => Language::Swift,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid language",
                ))
            }
        };

        client_outputs.push(Output { language, path });
    }

    let client_endpoint_dev = verify_string(&client, "endpoint_dev")?;
    let client_endpoint_prod = verify_string(&client, "endpoint_prod")?;

    let server_language = verify_string(&server_output, "language")?;
    let server_path = verify_string(&server_output, "path")?;

    let server_language = match server_language.as_str() {
        "rust" => Language::Rust,
        "swift" => Language::Swift,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid language",
            ))
        }
    };

    let client = ClientInfo {
        outputs: client_outputs,
        endpoint_dev: client_endpoint_dev,
        endpoint_prod: client_endpoint_prod,
    };
    let server = ServerInfo {
        output: Output {
            language: server_language,
            path: server_path,
        },
    };

    Ok(Info { client, server })
}

fn verify_table(table: &toml::Table, key: &str) -> Result<toml::Table, std::io::Error> {
    let value = table.get(key);
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Missing {key} configuration"),
        ));
    }
    let value = value.unwrap();

    let value = value.as_table();
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{key} configuration is not a table"),
        ));
    }

    Ok(value.unwrap().clone())
}

fn verify_array(table: &toml::Table, key: &str) -> Result<Vec<toml::Value>, std::io::Error> {
    let value = table.get(key);
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Missing {key} configuration"),
        ));
    }
    let value = value.unwrap();

    let value = value.as_array();
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{key} configuration is not an array"),
        ));
    }

    Ok(value.unwrap().clone())
}

fn verify_string(table: &toml::Table, key: &str) -> Result<String, std::io::Error> {
    let value = table.get(key);
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Missing {key} configuration"),
        ));
    }
    let value = value.unwrap();

    let value = value.as_str();
    if value.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{key} configuration is not a string"),
        ));
    }

    Ok(value.unwrap().to_string())
}
