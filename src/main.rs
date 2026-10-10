use std::{
    io::prelude::*,
    path::{Path, PathBuf},
    process::ExitCode,
};

use rlox::run;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        return ExitCode::from(64);
    }

    let result = if let Some(path) = args.next() {
        run_file(&PathBuf::from(path))
    } else {
        run_prompt()
    };

    if let Err(error) = result {
        println!("Error: {error}");
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}

fn run_file(path: &Path) -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        if let Some(line) = lines.next() {
            run(&line?);
        } else {
            break;
        }
    }

    Ok(())
}
