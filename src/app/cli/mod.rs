pub mod cmd;

pub fn run() -> anyhow::Result<()> {
    let opts = clap::Command::new("ruby_lsp")
        .version(crate::VERSION)
        .author("David K.")
        .about("A ruby LSP that actually works")
        .infer_subcommands(true)
        .subcommand_required(true)
        .subcommand(cmd::stdio::options())
        .get_matches();

    match opts.subcommand().unwrap() {
        ("stdio", stdio_opts) => Ok(cmd::stdio::run(&stdio_opts)?),
        _ => unreachable!(),
    }
}
