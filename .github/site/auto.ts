import * as fs from "fs/promises";
import * as fsSync from "fs";
import slugify from "slugify";
import YAML from "yaml";
import { Paths } from "./platform/paths.ts";
import * as civitai from "./platform/civitai.ts";
import type {ModelIndex} from './cmd/generate-index.ts'

const checkpoints: Array<number> = []
const loras: Array<number> = [

]

void (async function main() {
  if (fsSync.existsSync(Paths["~/temp"])) {
    await fs.rm(Paths["~/temp"], { recursive: true });
  }
  await fs.mkdir(Paths["~/temp"], { recursive: true });

  for (const model_id of loras) {
    const response = await civitai.civitAiModel(model_id)

    const name = slugify.default(convertToDashCase(response.name))
    const modelId = response.id
    const modelVersionName = response.modelVersions[0].name
    const modelVersionId = response.modelVersions[0].id
    const fileId = response.modelVersions[0].files[0].id
    const fileName = response.modelVersions[0].files[0].name

    const obj: ModelIndex = {
      // @ts-expect-error
      modelVersionName,
      fileName,
      type: 'lora',
      format: 'safetensors',
      source: {
        civitai: {
          model_id: modelId,
          model_version_id: modelVersionId,
          model_file_id: fileId,
        }
      }
    }
  
    const str = YAML.stringify(obj)
    await fs.writeFile(Paths["~/temp/"](`${name}@0.0.0.yml`), str, 'utf8')
  }
})();

function convertToDashCase(input: string): string {
  return input
    // Insert a dash before any uppercase letter that follows a lowercase letter or digit
    .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
    // Insert a dash before any uppercase letter that is followed by a lowercase letter
    // and preceded by an uppercase letter (handles sequences like "XMLParser" -> "XML-Parser")
    .replace(/([A-Z])([A-Z][a-z])/g, '$1-$2')
    // Convert to lowercase
    .toLowerCase();
}