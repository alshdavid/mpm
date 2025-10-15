use clap::Parser;

#[derive(Debug, Parser)]
pub struct RemoveCommand {
    /// Name of the model to add
    model: String,
}

pub async fn main(cmd: RemoveCommand) -> anyhow::Result<()> {
    dbg!(&cmd);
    Ok(())
}
