use std::{fs::File, io::Read, path::Path};

use clap::Parser;
use lexer::lexer::Lexer;

mod lexer;
mod shared;
mod tests;

#[derive(Parser)]
#[command(long_about = None)]
struct Config {
    pub file_path_to_main: String,
}

fn main() -> anyhow::Result<()> {
    let args = Config::parse();

    let file_path = args.file_path_to_main;

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);
    let formatted = contents.lines().map(|x| x.trim()).collect::<String>();
    let tokens = Lexer::new(formatted.trim().to_string()).collect();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
