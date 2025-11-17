use clap::Command;

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("nt2311-vn")
        .about("Rust echo")
        .get_matches();
}
