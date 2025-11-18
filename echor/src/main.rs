use clap::{Arg, ArgAction, Command, builder::ValueParser};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("nt2311-vn")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..)
                .value_parser(ValueParser::os_string()),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<std::ffi::OsString>("text")
        .unwrap()
        .map(|os_str| os_str.to_string_lossy().to_string())
        .collect();

    let omit_newline: bool = *matches.get_one::<bool>("omit_newline").unwrap_or(&false);

    let mut ending = "\n";

    if omit_newline {
        ending = "";
    }

    print!("{}{}", text.join(" "), ending);
}
