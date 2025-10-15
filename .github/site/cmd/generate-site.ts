import * as fs from "fs/promises";
import { Paths } from "../platform/paths.ts";

const html = /*html*/`
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Machine Learning Model Index</title>
</head>
<body>
  <h1>Model Package Manager</h1>
</body>
</html>
`

export async function main() {
  await fs.writeFile(
    Paths["~/dist/"](`index.html`),
    html,
    "utf8"
  );
}
