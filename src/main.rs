use clap::Parser;
use std::fs;
use std::io;
mod token;
mod scanner;

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


fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: i32, code: String, message: String) {
    println!("[line {}] Error{}: {}",line, code, message)
}
