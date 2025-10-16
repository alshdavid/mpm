// https://civitai.com/models/140272
// https://civitai.com/models/140272?modelVersionId=2262382
// https://civitai.com/api/download/models/2262382?type=Model&format=SafeTensor&size=pruned&fp=fp16

use std::path::PathBuf;

use url::Url;

use super::add::AddCommand;
use crate::cmd::add::AddResult;
use crate::platform::civitai::CivitAiFile;
use crate::platform::civitai::{self};
use crate::platform::format_name::format_name;
use crate::platform::os_string_ext::OsStringExt;
use crate::platform::path_ext::PathExt;

pub async fn main(cmd: &AddCommand) -> anyhow::Result<Option<AddResult>> {
    if cmd.model.contains("https://civitai.com/api/") {
        let url = Url::parse(&cmd.model)?;

        let Some(model_id) = url.path().split("/").into_iter().last() else {
            return Err(anyhow::anyhow!("Could not find model ID"));
        };

        let model = civitai::get_model_version(model_id, cmd.civitai_token.clone()).await?;

        let Some(file) = model.files.get(0) else {
            return Err(anyhow::anyhow!("Could not find file to download"));
        };

        let file_name = get_file_name(&cmd, &file)?;
        let file_stem = PathBuf::from(&file_name).try_file_stem()?;

        let mut download_url = file.download_url.clone();
        if let Some(token) = &cmd.civitai_token {
            download_url += format!("?token={}", token).as_str()
        }

        return Ok(Some(AddResult {
            download_url,
            download_url_safe: cmd.model.clone(),
            model_type: model.model.model_type.to_lowercase(),
            file_name,
            file_stem,
        }));
    }

    if cmd.model.contains("https://civitai.com") {
        let url = Url::parse(&cmd.model)?;

        let Some(model_id) = url.path().split("/").into_iter().last() else {
            return Err(anyhow::anyhow!("Could not find model ID"));
        };

        let model_version_id = match url.query_pairs().find(|(v, _)| v == "modelVersionId") {
            Some((_, model_version_id)) => Some(model_version_id.to_string()),
            None => None,
        };

        let model = civitai::get_model(model_id, cmd.civitai_token.clone()).await?;

        // dbg!(&model);
        let model_version = if let Some(model_version_id) = model_version_id {
            let Some(model_version) = model
                .model_versions
                .iter()
                .find(|m| format!("{}", m.id) == model_version_id)
            else {
                return Err(anyhow::anyhow!("Could not find latest model version"));
            };
            model_version
        } else {
            let Some(model_version) = model.model_versions.get(0) else {
                return Err(anyhow::anyhow!("Could not find latest model version"));
            };
            model_version
        };

        for file in &model_version.files {
            if !file.primary {
                continue;
            }

            let mut download_url = file.download_url.clone();
            if let Some(token) = &cmd.civitai_token {
                download_url += format!("?token={}", token).as_str()
            }

            let file_name = get_file_name(&cmd, &file)?;
            let file_stem = PathBuf::from(&file_name).try_file_stem()?;

            return Ok(Some(AddResult {
                download_url,
                download_url_safe: cmd.model.clone(),
                model_type: model.model_type.to_lowercase(),
                file_name,
                file_stem,
            }));
        }
    }

    return Ok(None);
}

fn get_file_name(
    cmd: &AddCommand,
    file: &CivitAiFile,
) -> anyhow::Result<String> {
    if let Some(filename) = cmd.name.clone() {
        let Some(extension) = PathBuf::from(&file.name)
            .extension()
            .map(|v| v.to_os_string())
        else {
            return Ok(format_name(&filename));
        };

        if filename.ends_with(&extension.clone().try_to_string()?) {
            return Ok(format_name(&filename));
        }

        return Ok(format!(
            "{}.{}",
            format_name(&filename),
            extension.try_to_string()?
        ));
    }

    Ok(format_name(&file.name))
}
