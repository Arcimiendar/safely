mod args;
mod subprocess;
mod limitations;

use clap::Parser;
use anyhow::Result;

use args::Args;
use subprocess::run_subprocess;
use limitations::prepare_limitations;

fn main() -> Result<()> {
    let args = Args::parse();
    prepare_limitations(&args.config)?;
    run_subprocess(&args.shell, &args.command)?;

    Ok(())
}
