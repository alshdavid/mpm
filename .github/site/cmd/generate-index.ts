import * as fs from "node:fs/promises";
import * as path from "node:path";
import YAML from "yaml";
import * as civitai from "../platform/civitai.ts";
import { Paths } from "../platform/paths.ts";
import { sortEntries } from "../platform/sort.ts";

export type ModelIndex = {
  type: "checkpoint" | "lora";
  format: "safetensors";
  source: {
    civitai: {
      model_id: number;
      model_version_id: number;
      model_file_id: number;
    };
  };
};

export type ModelManifestEntry = {
  type: "checkpoint" | "lora";
  format: "safetensors";
  source: "civitai" | "huggingface" | "direct";
  downloadUrl: string;
};
export type ModelManifest = Record<string, ModelManifestEntry>;

export async function main() {
  const manifest: ModelManifest = {};

  const promises: Array<Promise<void>> = [];

  for (const modelType of await fs.readdir(Paths["~/models"])) {
    for (const model of await fs.readdir(Paths["~/models/"](modelType))) {
      promises.push(
        new Promise((res) => setTimeout(res, 0)).then(async () => {
          const { name: modelName } = path.parse(model);
          console.log(modelName);
          const file = await fs.readFile(Paths["~/models/"](modelType, model), "utf8");
          const yaml: ModelIndex = YAML.parse(file);
          const config = yaml.source.civitai;

          const response = await civitai.civitAiModel(config.model_id);
          const modelVersion = response.modelVersions.find(
            (version) => version.id === config.model_version_id
          );
          if (!modelVersion) {
            throw new Error(`Unable to find ${model}:${config.model_version_id}`);
          }

          const modelFile = modelVersion.files.find(
            (file) => file.id === config.model_file_id
          );
          if (!modelFile) {
            throw new Error(
              `Unable to find ${model}:${config.model_version_id}:${config.model_file_id}`
            );
          }

          manifest[modelName] = {
            downloadUrl: modelFile.downloadUrl,
            source: "civitai",
            type: yaml.type,
            format: yaml.format,
          };
        })
      );
    }
  }

  await Promise.all(promises);

  for (const [specifier, meta] of Object.entries(manifest)) {
    await fs.writeFile(
      Paths["~/dist/"](`${specifier}.json`),
      JSON.stringify(meta, null, 2),
      "utf8"
    );
  }

  const versionManifest: Record<string, Array<string>> = {};

  for (const [specifier, meta] of Object.entries(manifest)) {
    const [modelName, modelVersion] = specifier.split("@");
    versionManifest[modelName] = versionManifest[modelName] || [];
    versionManifest[modelName].push(modelVersion);
    versionManifest[modelName].sort(sortEntries);
  }

  for (const [specifier, meta] of Object.entries(versionManifest)) {
    await fs.writeFile(
      Paths["~/dist/"](`${specifier}.json`),
      JSON.stringify(meta, null, 2),
      "utf8"
    );
  }

  await fs.writeFile(
    Paths["~/dist/"](`index.json`),
    JSON.stringify(Object.keys(versionManifest).sort(sortEntries).reverse(), null, 2),
    "utf8"
  );
}
