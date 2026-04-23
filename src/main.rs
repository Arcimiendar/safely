pub mod args;

use std::process::Command;
use clap::Parser;
use args::Args;
use shlex::try_join;
use anyhow::Result;

fn prepare_limitations(args: &Args) -> Result<()> {
    Ok(())
}

fn run_subprocess(args: &Args) -> Result<()> {
    let command = try_join(args.rest.iter().map(|s| s.as_str()))?;
    Command::new(&args.shell)
        .args(["-c", &command])
        .spawn()?
        .wait()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    prepare_limitations(&args)?;
    run_subprocess(&args)?;

    Ok(())
}
