//! The CLI tool for executing the scriptkiddie engine on a JavaScript file

use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use scriptkiddie_lexer::lexer::Lexer;
use scriptkiddie_parser::parser::Parser as ASTParser;

/// The config for running scriptkiddie
#[derive(Parser, Debug)]
struct Config {
    /// The path of the file to execute
    file: PathBuf,
}

impl Config {
    /// Reads all file lines to a string
    pub fn lines(self) -> String {
        let mut file = File::open(self.file).expect("The file supplied does not exist");
        let mut lines = String::new();

        file.read_to_string(&mut lines)
            .expect("Failed to read file!");

        lines
    }
}

fn main() {
    let lines = Config::parse().lines();
    let lexer = Lexer::new(lines);
    let tokens: Vec<_> = lexer.collect();
    let mut parser = ASTParser::new(&tokens);
    let _ast = parser.parse_program();
}
