use clap::Parser;

#[derive(Debug, Parser)]
pub struct FetchCommand {
    /// Name of the model to add
    model: String,
}

pub async fn main(cmd: FetchCommand) -> anyhow::Result<()> {
    dbg!(&cmd);
    Ok(())
}
