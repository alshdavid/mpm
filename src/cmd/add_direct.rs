use super::add::AddCommand;
use super::add::AddResult;

pub async fn main(_cmd: &AddCommand) -> anyhow::Result<Option<AddResult>> {
    Ok(None)
}
