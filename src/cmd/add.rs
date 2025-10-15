use clap::Parser;

#[derive(Debug, Parser)]
pub struct AddCommand {
    /// Name of the model to add
    model: String,
}

pub async fn main(cmd: AddCommand) -> anyhow::Result<()> {
    dbg!(&cmd);
    Ok(())
}
