use clap::Parser;
use std::fs;
use std::io;
mod token;
mod scanner;
mod error;
mod expressions;
mod parser;

#[derive(Parser,Default,Debug)]
struct Args {

    script: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.script {
        Some(path) => run_file(path),
        None => run_prompt(),
    }
}

fn run_prompt() {
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        run(line)
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Error reading script");    
    run(contents);
}

fn run(code: String) {
    let mut scanner = scanner::Scanner::new(code);
    let tokens = scanner.scan_tokens();
    for token in tokens.iter() {
        println!("{}", token.to_string());
    }
}
