mod args;
mod subprocess;

use clap::Parser;
use anyhow::Result;

use args::Args;
use subprocess::run_subprocess;

fn prepare_limitations(args: &Args) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    prepare_limitations(&args)?;
    run_subprocess(&args.shell, &args.command)?;

    Ok(())
}
