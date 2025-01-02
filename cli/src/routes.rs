use std::path::PathBuf;

use parser::Parser;

pub fn parse(path: &str) -> Result<Vec<parser::ast::FileNode>, std::io::Error> {
    // get all files in `path` that end with .fen
    let file_names = std::fs::read_dir(path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension()?.to_str()? == "fen" {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<PathBuf>>();

    // parse each file
    let mut routes = vec![];
    for file_name in file_names {
        let file_contents = std::fs::read_to_string(&file_name)?;
        let mut parser = Parser::new(&file_contents);
        let ast = parser.parse();
        if let Err(e) = ast {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Error parsing file",
            ));
        }
        routes.push(ast.unwrap());
    }

    Ok(routes)
}
