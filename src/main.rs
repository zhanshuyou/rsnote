use rsnote_cli::cli;
fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
