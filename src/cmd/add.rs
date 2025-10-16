use std::path::PathBuf;

use clap::Parser;
use indicatif::ProgressBar;

use crate::platform::download::download_file;
use crate::platform::models_yaml::ModelsYaml;

#[derive(Debug)]
pub struct AddResult {
    pub download_url: String,
    pub download_url_safe: String,
    pub model_type: String,
    pub file_name: String,
    pub file_stem: String,
}

#[derive(Debug, Parser, Clone)]
pub struct AddCommand {
    /// Name or URL of the model to add
    pub model: String,

    /// What to name the model
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// Target path to download file to
    #[arg(short = 'o', long = "out-dir")]
    pub out_dir: Option<PathBuf>,

    #[arg(env = "CIVITAI_TOKEN")]
    pub civitai_token: Option<String>,
}

pub async fn main(cmd: AddCommand) -> anyhow::Result<()> {
    // dbg!(&cmd);
    println!("Resolving:      \"{}\"", cmd.model);

    let mut download_url = None::<AddResult>;

    // https://civitai.com/models/140272
    // https://civitai.com/models/140272?modelVersionId=2262382
    // https://civitai.com/api/download/models/2262382?type=Model&format=SafeTensor&size=pruned&fp=fp16
    if cmd.model.starts_with("https://civitai.com") {
        download_url = super::add_civit::main(&cmd).await?;
    }
    // https://example.com/model.safetensors
    else if cmd.model.starts_with("https://") {
        download_url = super::add_direct::main(&cmd).await?;
    }
    // pony@6.0.0
    else {
        download_url = super::add_known::main(&cmd).await?;
    }

    let Some(AddResult {
        download_url,
        download_url_safe,
        model_type,
        file_name,
        file_stem,
    }) = download_url
    else {
        return Err(anyhow::anyhow!("Unable to resolve download URL"));
    };
    println!("Resolved:       \"{}\"", file_stem);

    let out_dir = match cmd.out_dir {
        Some(out_dir) => out_dir,
        None => std::env::current_dir()?
            .join("models")
            .join(match model_type.as_str() {
                "lora" => "loras",
                "checkpoint" => "checkpoints",
                other => &other,
            }),
    };

    let out_file = out_dir.join(&file_name);
    println!("Downloading to: {:?}", out_file);
    if std::fs::exists(&out_file)? {
        println!("Already downloaded");
        return Ok(());
    }

    if !std::fs::exists(&out_dir)? {
        std::fs::create_dir_all(&out_dir)?;
    }

    // Download Model
    let mut rx = download_file(&download_url, &out_file);

    let pb = ProgressBar::new(100);
    while let Some(update) = rx.recv().await {
        let update = match update {
            Ok(update) => update,
            Err(err) => return Err(err),
        };
        pb.set_position(update.percent.round() as u64);
    }
    pb.finish();

    // Update yaml file
    let models_yaml_path = ModelsYaml::find_or_default()?;
    let mut models_yaml = ModelsYaml::parse_file(&models_yaml_path)
        .await
        .unwrap_or_default();
    models_yaml.models.insert(file_stem, download_url_safe);
    models_yaml.write_file(&models_yaml_path).await?;

    Ok(())
}
