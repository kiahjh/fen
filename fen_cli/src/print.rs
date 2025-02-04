use colored::Colorize;

pub fn info(text: &str) {
    println!("{}", text.truecolor(150, 150, 150).italic());
}

pub fn success(text: &str) {
    println!("{} {}", "✔".green(), text);
}
