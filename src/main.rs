#![deny(unused_crate_dependencies)]
use clap::Parser;
use clap::Subcommand;

mod cmd;
mod platform;

#[derive(Debug, Subcommand)]
pub enum RootCommandType {
    /// Add a model to the current project
    Add(cmd::add::AddCommand),
    /// Install all models defined in the models.yml
    Install(cmd::install::InstallCommand),
}

#[derive(Parser, Debug)]
pub struct RootCommand {
    #[clap(subcommand)]
    pub command: RootCommandType,
    #[arg(env = "CIVITAI_TOKEN")]
    pub civitai_token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = RootCommand::parse();

    match args.command {
        RootCommandType::Add(cmd) => cmd::add::main(cmd).await,
        RootCommandType::Install(cmd) => cmd::install::main(cmd).await,
    }
}
