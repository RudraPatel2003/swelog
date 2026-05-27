#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

try {
  main();
} catch (error) {
  console.error("An unknown error occurred while running swelog");
  console.error(error);

  process.exit(1);
}

function main() {
  const executablePath = getExecutablePath();

  ensureExecutableExists(executablePath);

  const result = spawnSync(executablePath, process.argv.slice(2), {
    stdio: "inherit",
  });

  if (result.error) {
    fail(result.error.message);
  }

  if (result.signal) {
    process.kill(process.pid, result.signal);
  }

  process.exit(result.status ?? 1);
}

function getExecutablePath() {
  const executableName = process.platform === "win32" ? "swelog.exe" : "swelog";

  return join(getCurrentDirectory(), "..", "vendor", executableName);
}

function getCurrentDirectory() {
  return dirname(fileURLToPath(import.meta.url));
}

function ensureExecutableExists(executablePath) {
  if (!existsSync(executablePath)) {
    fail(
      "swelog binary is missing. Reinstall the package without --ignore-scripts.",
    );
  }
}

function fail(message) {
  console.error(`Error: ${message}`);

  process.exit(1);
}
