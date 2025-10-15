use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ModelMeta {
    pub download_url: String,
    pub source: String,
    pub kind: String,
    pub format: String,
}

pub async fn get_model_details(specifier: &str) -> anyhow::Result<ModelMeta> {
    let url = format!("https://alshdavid.github.io/mpm/{}.json", specifier);
    let response = reqwest::get(url).await?;
    if response.status() != 200 {
        return Err(anyhow::anyhow!("Unable to resolve package {}", specifier));
    }

    Ok(serde_json::from_slice(&(response.bytes().await?))?)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub versions: Vec<String>,
}

pub async fn resolve_model_info(specifier: &str) -> anyhow::Result<ModelInfo> {
    let url = format!("https://alshdavid.github.io/mpm/{}.json", specifier);
    let response = reqwest::get(url).await?;
    if response.status() != 200 {
        return Err(anyhow::anyhow!("Unable to resolve package {}", specifier));
    }

    Ok(serde_json::from_slice(&(response.bytes().await?))?)
}
