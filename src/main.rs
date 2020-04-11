use std::env;
use std::fs;
use std::io::{self, Write};

pub mod token;

pub mod scanner;
use scanner::Scanner;

pub struct RustyLocks {
    had_error: bool
}

impl RustyLocks {
    pub fn new() -> RustyLocks {
        RustyLocks {
            had_error: false
        }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

        self.run(contents);
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().expect("error flushing stdout");
            io::stdin().read_line(&mut input).expect("error reading from stdin");
            self.run(String::from(input.trim()));
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();

        for token in tokens.iter() {
           println!("{}", token);
        }

        if self.had_error {
            panic!("we had an error, rusty-locks exiting!")
        }
    }

    fn error(&mut self, line: i32, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: i32, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rl = RustyLocks::new();

    match args.len() {
        1 => { rl.run_prompt(); },
        2 => { rl.run_file(args[1].clone()); },
        _ => { println!("Usage: rusty-locks [script]"); }
    };
}
