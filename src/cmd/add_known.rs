use super::add::AddCommand;
use super::add::AddResult;
use crate::platform::index;

pub async fn main(cmd: &AddCommand) -> anyhow::Result<Option<AddResult>> {
    let cmd = cmd.clone();

    let specifier = match cmd.model.contains("@") {
        true => cmd.model,
        false => {
            let info = index::resolve_model_info(&cmd.model).await?;
            let Some(latest) = info.versions.get(0) else {
                return Err(anyhow::anyhow!(
                    "Unable to resolve version for: {}",
                    cmd.model
                ));
            };
            format!("{}@{}", cmd.model, latest)
        }
    };

    let found = index::get_model_details(&specifier).await?;

    let mut download_url = found.download_url.clone();
    if let Some(token) = &cmd.civitai_token {
        download_url += format!("?token={}", token).as_str()
    }

    Ok(Some(AddResult {
        download_url,
        download_url_safe: found.download_url,
        model_type: found.kind.to_lowercase(),
        file_name: format!("{}.{}", specifier, found.format),
        file_stem: specifier,
    }))
}
