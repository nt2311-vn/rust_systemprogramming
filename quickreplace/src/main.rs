use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    print_usage();
}

fn print_usage() {
    eprintln!(
        "{} - change occurences of one string to another",
        "quickreplace".green()
    )
}
