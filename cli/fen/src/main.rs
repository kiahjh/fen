use parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: fen <PATH>");
        std::process::exit(1);
    }
    let dir_path = &args[1];
    let dir = std::fs::read_dir(dir_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let file_contents = dir
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let contents = std::fs::read_to_string(&path).unwrap_or_else(|e| {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                });
                Some(contents)
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let ast_json = file_contents
        .iter()
        .map(|entry| {
            let mut parser = Parser::new(entry);
            let ast = parser.parse().unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                std::process::exit(1);
            });
            let json = serde_json::to_string_pretty(&ast).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                std::process::exit(1);
            });

            NameAndJson {
                name: ast.name.clone(),
                json,
            }
        })
        .collect::<Vec<NameAndJson>>();

    for NameAndJson { name, json } in ast_json {
        println!("{name}\n{json}\n\n");
    }
}

struct NameAndJson {
    name: String,
    json: String,
}
