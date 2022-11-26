const execa = require("execa");
const path = require("path");
const fs = require("fs");

const { getWorkspaces: getWorkspacesNodeJS } = require("workspace-tools");
const { getWorkspaces: getWorkspacesRust } = require("../index");

const largeMonorepo = "https://github.com/vsavkin/large-monorepo";

async function main() {
  const tmpDir = path.join(process.cwd(), "_tmp");

  if (!fs.existsSync(tmpDir)) {
    await execa("git", ["clone", largeMonorepo, tmpDir]);
  }

  console.time("workspace-tools time");
  getWorkspacesNodeJS(path.join(process.cwd(), "_tmp"));
  console.timeEnd("workspace-tools time");

  console.time("workspace-tools time");
  getWorkspacesRust(path.join(process.cwd(), "_tmp"));
  console.timeEnd("workspace-tools time");
}

main().catch((e) => {
  console.error(e);
  process.exitCode = 1;
});
