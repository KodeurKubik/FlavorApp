import { existsSync, readFileSync, rmSync, writeFileSync } from "fs";

const VERSION = readFileSync("./Cargo.toml", "utf-8")
  .split("version = ")[1]
  .split("\n")[0]
  .replaceAll('"', "");

console.log(`Parsed version ${VERSION}`);

const URL_PREFIX = `https://github.com/KodeurKubik/FlavorApp/releases/download/v${VERSION}`;

let LATEST: {
  version: string;
  notes: string;
  pub_date: string;
  platforms: { [p: string]: { signature: string; url: string } };
} = {
  version: VERSION,
  notes: prompt("Release Notes:") || "",
  pub_date: new Date().toISOString(),
  platforms: {},
};

const platforms: { [p: string]: string } = {
  "windows-x86_64": "FlavorApp.exe",
  "darwin-aarch64": "FlavorApp.app.tar.gz",
};

Object.keys(platforms).forEach((p) => {
  if (existsSync(`./build/out/${platforms[p]}`)) {
    LATEST.platforms[p] = {
      url: `${URL_PREFIX}/${platforms[p]}`,
      signature: readFileSync(`./build/out/${platforms[p]}.sig`, "utf-8"),
    };

    rmSync(`./build/out/${platforms[p]}.sig`);
  }
});

writeFileSync(
  "./build/out/latest.json",
  JSON.stringify(LATEST, null, 2),
  "utf-8"
);
