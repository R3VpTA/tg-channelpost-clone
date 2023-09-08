use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLI: Cli = parse_args();
}

#[derive(Parser)]
pub struct Cli {
    /// Bot's Dev ID
    #[arg(long)]
    pub dev_id: i64,

    /// Bot Token
    #[arg(long)]
    pub token: String,
}

fn parse_args() -> Cli {
    Cli::parse()
}
