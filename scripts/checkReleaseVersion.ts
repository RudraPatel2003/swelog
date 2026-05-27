import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

try {
  main();
} catch (error) {
  console.error("An unknown error occurred while checking the release version");
  console.error(error);

  process.exit(1);
}

function main() {
  const releaseTag = getReleaseTagFromArgs();

  const releaseVersion = getReleaseVersionFromTag(releaseTag);

  const rustCliVersion = getRustCliVersion();

  const npmCliVersion = getNpmCliVersion();

  if (rustCliVersion !== releaseVersion) {
    fail(
      `cli crate version ${rustCliVersion} does not match release tag ${releaseTag}`,
    );
  }

  if (npmCliVersion !== releaseVersion) {
    fail(
      `npm package version ${npmCliVersion} does not match release tag ${releaseTag}`,
    );
  }

  console.log(`Release tag ${releaseTag} matches CLI and npm package versions`);
}

function getReleaseTagFromArgs(): string {
  const releaseTag = process.argv[2];

  if (!releaseTag) {
    fail("Usage: npx tsx ./scripts/checkReleaseVersion.ts <release-tag>");
  }

  return releaseTag;
}

function getReleaseVersionFromTag(releaseTag: string): string {
  if (!releaseTag.startsWith("v")) {
    fail("Release tag must look like vX.Y.Z");
  }

  return releaseTag.slice(1);
}

function getRustCliVersion(): string {
  const packageId = execFileSync("cargo", ["pkgid", "-p", "cli"], {
    encoding: "utf8",
  }).trim();

  const version = packageId.split("#").at(-1) ?? "";

  if (!version) {
    fail(`Could not parse version from package id: ${packageId}`);
  }

  return version;
}

function getNpmCliVersion(): string {
  const packageJson = JSON.parse(
    readFileSync("./npm/package.json", "utf8"),
  ) as Record<string, unknown>;

  if (!packageJson.version || typeof packageJson.version !== "string") {
    fail("npm/package.json is missing a version");
  }

  return packageJson.version;
}

function fail(message: string): never {
  console.error(`Error: ${message}`);

  const EXIT_CODE = 1;

  process.exit(EXIT_CODE);
}
