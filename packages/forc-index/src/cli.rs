use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "forc index", about = "Fuels Indexer Orchestrator", version)]
pub struct Opt {
    /// The command to run
    #[clap(subcommand)]
    pub command: ForcIndex,
}

#[derive(Subcommand, Debug)]
pub enum ForcIndex {}

pub async fn run_cli() -> Result<(), anyhow::Error> {
    let _opt = Opt::parse();

    Ok(())
}
