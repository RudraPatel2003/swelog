#!/usr/bin/env node

import {
  chmodSync,
  existsSync,
  mkdirSync,
  readFileSync,
  renameSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

try {
  await main();
} catch (error) {
  console.error("An unknown error occurred while installing swelog");
  console.error(error);

  process.exit(1);
}

async function main() {
  const packageJson = getPackageJson();

  const vendorDirectory = getVendorDirectory();

  const destination = getDestination(vendorDirectory);

  const assetName = getAssetName();

  const downloadUrl = getDownloadUrl(packageJson.version, assetName);

  const temporaryDestination = `${destination}.download`;

  mkdirSync(vendorDirectory, { recursive: true });

  if (existsSync(temporaryDestination)) {
    rmSync(temporaryDestination, { force: true });
  }

  await download(downloadUrl, temporaryDestination, packageJson);

  renameSync(temporaryDestination, destination);

  chmodSync(destination, 0o755);

  console.log(
    `Installed swelog ${packageJson.version} for ${process.platform} ${process.arch}`,
  );
}

function getPackageJson() {
  const packageJson = JSON.parse(
    readFileSync(join(getPackageDirectory(), "package.json"), "utf8"),
  );

  if (!packageJson.name || !packageJson.version) {
    throw new Error("npm/package.json is missing a name or version");
  }

  return packageJson;
}

function getPackageDirectory() {
  return join(getCurrentDirectory(), "..");
}

function getCurrentDirectory() {
  return dirname(fileURLToPath(import.meta.url));
}

function getVendorDirectory() {
  return join(getPackageDirectory(), "vendor");
}

function getDestination(vendorDirectory) {
  const executableName = process.platform === "win32" ? "swelog.exe" : "swelog";

  return join(vendorDirectory, executableName);
}

function getAssetName() {
  if (process.platform === "linux" && process.arch === "x64") {
    return "swelog-x86_64-unknown-linux-gnu";
  }

  if (process.platform === "darwin" && process.arch === "x64") {
    return "swelog-x86_64-apple-darwin";
  }

  if (process.platform === "darwin" && process.arch === "arm64") {
    return "swelog-aarch64-apple-darwin";
  }

  if (process.platform === "win32" && process.arch === "x64") {
    return "swelog-x86_64-pc-windows-msvc.exe";
  }

  throw new Error(`Unsupported platform: ${process.platform} ${process.arch}`);
}

function getDownloadUrl(version, assetName) {
  return `https://github.com/RudraPatel2003/swelog/releases/download/v${version}/${assetName}`;
}

async function download(url, filePath, packageJson) {
  const response = await fetch(url, {
    headers: {
      "User-Agent": `${packageJson.name}/${packageJson.version}`,
    },
  });

  if (!response.ok) {
    throw new Error(`Download failed with HTTP ${response.status}: ${url}`);
  }

  const binary = Buffer.from(await response.arrayBuffer());

  writeFileSync(filePath, binary, { mode: 0o755 });
}
