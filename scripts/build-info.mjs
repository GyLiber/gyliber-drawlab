import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

// Public build identity only; never serialize environment variables or secrets.
const commit = execFileSync("git", ["rev-parse", "HEAD"], {
  encoding: "utf8",
}).trim();
if (!/^[a-f0-9]{40}$/.test(commit)) throw new Error("Invalid build commit");
const { version } = JSON.parse(readFileSync("web/package.json", "utf8"));
writeFileSync(
  "web/dist/build-info.json",
  JSON.stringify({ version, commit, scope: "laboratory-preview" }, null, 2) + "\n",
);
