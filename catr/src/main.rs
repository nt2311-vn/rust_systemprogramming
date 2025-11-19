use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("nt2311-vn")
        .about("Rust cat")
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number lines")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("Number nonblank lines")
                .action(ArgAction::SetFalse),
        )
        .get_matches();
}
