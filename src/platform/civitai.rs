use std::path::PathBuf;

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

    Ok(download_file(
        &format!("{}", url),
        &options.download_to.try_to_string()?,
    ))
}
