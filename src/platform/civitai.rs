use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use url::Url;

use super::download::download_file;
use crate::platform::download::DownloadProgressReceiver;
use crate::platform::path_ext::PathExt;

pub struct DownloadModelOptions {
    pub token: Option<String>,
    pub url: String,
    pub download_to: PathBuf,
}

pub async fn download_model(
    options: DownloadModelOptions
) -> anyhow::Result<DownloadProgressReceiver> {
    let mut url = Url::parse(&options.url)?;

    if let Some(token) = options.token {
        url.query_pairs_mut().append_pair("token", token.as_str());
    }

    if !std::fs::exists(&options.download_to.try_parent()?)? {
        std::fs::create_dir_all(&options.download_to.try_parent()?)?;
    }

    Ok(download_file(&format!("{}", url), &options.download_to))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CivitAiFile {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub download_url: String,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CivitAiModelVersion {
    pub id: u64,
    pub index: u32,
    pub name: String,
    // pub description: Option<String>,
    pub download_url: String,
    pub files: Vec<CivitAiFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CivitAiModel {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub model_type: String,
    pub model_versions: Vec<CivitAiModelVersion>,
}

pub async fn get_model(
    id: &str,
    token: Option<String>,
) -> anyhow::Result<CivitAiModel> {
    let mut url = format!("https://civitai.com/api/v1/models/{}", id);
    if let Some(token) = token {
        url += format!("?token={}", token).as_str()
    }

    let response = reqwest::get(&url).await?;
    if response.status() != 200 {
        return Err(anyhow::anyhow!(
            " Unable to resolve model {}\nURL:    {}\nStatus: {}",
            id,
            url,
            response.status()
        ));
    }

    Ok(serde_json::from_slice(&(response.bytes().await?))?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelVersionResponse {
    pub id: u64,
    pub model_id: u64,
    pub name: String,
    pub download_url: String,
    pub model: GetModelVersionResponseModel,
    pub files: Vec<CivitAiFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelVersionResponseModel {
    #[serde(rename = "type")]
    pub model_type: String,
}

pub async fn get_model_version(
    id: &str,
    token: Option<String>,
) -> anyhow::Result<GetModelVersionResponse> {
    let mut url = format!("https://civitai.com/api/v1/model-versions/{}", id);
    if let Some(token) = token {
        url += format!("?token={}", token).as_str()
    }

    let response = reqwest::get(&url).await?;
    if response.status() != 200 {
        return Err(anyhow::anyhow!(
            " Unable to resolve model {}\nURL:    {}\nStatus: {}",
            id,
            url,
            response.status()
        ));
    }

    Ok(serde_json::from_slice(&(response.bytes().await?))?)
}
