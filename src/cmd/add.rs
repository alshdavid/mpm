use std::path::PathBuf;

use clap::Parser;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::platform::civitai;
use crate::platform::index;
use crate::platform::models_yaml::ModelsYaml;
use crate::platform::os_string_ext::*;
use crate::platform::path_ext::*;

#[derive(Debug, Parser)]
pub struct AddCommand {
    /// Name of the model to add
    pub model: String,

    /// Target path to download file to
    #[arg(short = 'o', long = "out-dir")]
    pub out_dir: Option<PathBuf>,

    #[arg(env = "CIVITAI_TOKEN")]
    pub civitai_token: Option<String>,
}

pub async fn main(cmd: AddCommand) -> anyhow::Result<()> {
    // dbg!(&cmd);

    let models_yaml_path = ModelsYaml::find_or_default()?;
    let mut models_yaml = ModelsYaml::parse_file(&models_yaml_path)
        .await
        .unwrap_or_default();

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
    // dbg!(&found);

    let out_dir = match cmd.out_dir {
        Some(out_dir) => out_dir,
        None => std::env::current_dir()?
            .join("models")
            .join(match found.kind.as_str() {
                "lora" => "loras",
                "checkpoint" => "checkpoints",
                other => &other,
            }),
    };

    let out_file = out_dir.join(format!("{}.{}", specifier, found.format));

    let Some((model_name, model_version)) = specifier.split_once("@") else {
        return Err(anyhow::anyhow!("Failed to create specifier"));
    };

    models_yaml
        .models
        .insert(model_name.to_string(), model_version.to_string());

    println!("Downloading to: {:?}", out_file);
    if std::fs::exists(&out_file)? {
        println!("Already downloaded");
        return Ok(());
    }

    let mut rx = civitai::download_model(civitai::DownloadModelOptions {
        token: cmd.civitai_token.clone(),
        url: found.download_url,
        download_to: out_file,
    })
    .await?;

    let pb = ProgressBar::new(100);
    while let Some(Ok(update)) = rx.recv().await {
        pb.set_position(update.percent.round() as u64);
    }
    pb.finish();

    models_yaml.write_file(&models_yaml_path).await?;
    Ok(())
}
