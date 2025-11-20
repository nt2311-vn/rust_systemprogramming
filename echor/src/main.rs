use clap::Parser;

#[derive(Debug, Parser)]
#[command(version = "0.1.0", author = "nt2311-vn", about = "Rust echo")]
/// Rust version of echo
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    )
}
