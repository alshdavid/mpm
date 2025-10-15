import * as fs from "fs/promises";
import * as fsSync from "fs";
import { Paths } from "./platform/paths.ts";
import * as generateIndex from "./cmd/generate-index.ts";
import * as generateSite from "./cmd/generate-site.ts";

void (async function main() {
  if (fsSync.existsSync(Paths["~/dist"])) {
    await fs.rm(Paths["~/dist"], { recursive: true });
  }
  await fs.mkdir(Paths["~/dist"], { recursive: true });

  await generateIndex.main()
  await generateSite.main()
})();
