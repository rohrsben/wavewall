pub mod subcommands;

pub use subcommands::Subcommands;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Where to save the result image
    #[arg(long)]
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Subcommands>
}

