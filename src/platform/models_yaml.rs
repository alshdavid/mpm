use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::platform::path_ext::PathExt;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ModelsYaml {
    pub models: HashMap<String, String>,
}

impl ModelsYaml {
    pub async fn parse_file(path: &Path) -> anyhow::Result<Self> {
        if !(fs::exists(path)?) {
            return Err(anyhow::anyhow!("DoesNotExist; {:?}", {}));
        }
        let bytes = tokio::fs::read_to_string(path).await?;
        Self::parse(&bytes)
    }

    pub fn parse(data: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(data)?)
    }

    pub fn find_or_default() -> anyhow::Result<PathBuf> {
        if let Ok(Some(path)) = Self::find() {
            return Ok(path);
        }
        Self::default_path()
    }

    pub fn find() -> anyhow::Result<Option<PathBuf>> {
        let cwd = std::env::current_dir()?;
        for file in cwd.find_ancestor_file(&PathBuf::from("models.yml"))? {
            return Ok(Some(file));
        }

        for file in cwd.find_ancestor_file(&PathBuf::from("models.yaml"))? {
            return Ok(Some(file));
        }

        Ok(None)
    }

    pub fn default_path() -> anyhow::Result<PathBuf> {
        Ok(std::env::current_dir()?.join("models.yml"))
    }

    pub async fn write_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<()> {
        let bytes = serde_yaml::to_string(&self)?;
        Ok(tokio::fs::write(path, bytes.as_bytes()).await?)
    }
}
