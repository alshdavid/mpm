use clap::Parser;

use crate::platform::models_yaml::ModelsYaml;

#[derive(Debug, Parser)]
pub struct InstallCommand {
    #[arg(env = "CIVITAI_TOKEN")]
    pub civitai_token: Option<String>,
}

pub async fn main(cmd: InstallCommand) -> anyhow::Result<()> {
    let models_yaml_path = ModelsYaml::find_or_default()?;
    let models_yaml = ModelsYaml::parse_file(&models_yaml_path)
        .await
        .unwrap_or_default();

    for (model_name, model_version) in models_yaml.models {
        crate::cmd::add::main(crate::cmd::add::AddCommand {
            model: format!("{}@{}", model_name, model_version),
            out_dir: None,
            civitai_token: cmd.civitai_token.clone(),
        })
        .await?;

        println!("")
    }

    Ok(())
}
