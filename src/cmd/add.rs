use clap::Parser;

use crate::platform::index;

#[derive(Debug, Parser)]
pub struct AddCommand {
    /// Name of the model to add
    model: String,
}

pub async fn main(cmd: AddCommand) -> anyhow::Result<()> {
    let found = index::get_model_details(&cmd.model).await?;
    dbg!(&found);
    Ok(())
}

