use crate::scanner::Scanner;

mod scanner;

pub fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    for token in &scanner.tokens {
        println!("{token:?}");
    }

    for error in &scanner.errors {
        println!("ERROR: {error:?}");
    }
}
