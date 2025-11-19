use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

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
