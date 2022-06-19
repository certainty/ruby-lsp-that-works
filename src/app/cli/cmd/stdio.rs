pub fn options<'a>() -> clap::Command<'a> {
    clap::Command::new("stdio")
        .alias("s")
        .about("Run in stdio mode")
}

pub fn run(_opts: &clap::ArgMatches) -> anyhow::Result<()> {
    eprintln!("Starting LSP in stdio mode");
    Ok(())
}
