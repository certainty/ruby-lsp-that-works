use ruby_lsp::app::cli;

fn main() {
    pretty_env_logger::init();

    match cli::run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}
