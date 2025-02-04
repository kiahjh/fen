#![allow(unused_variables)]
#![allow(dead_code)]

use colored::Colorize;
use interface::run;

mod codegen;
mod config;
mod interface;
mod print;
mod routes;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let res = run();

    if let Err(e) = res {
        eprintln!("{} Error: {}\n", "✘".red(), e);
    }
}
