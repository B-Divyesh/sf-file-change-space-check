import { copyFile, mkdir } from "node:fs/promises";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const binary = resolve(root, "target/release/fcsc");
const artifact = resolve(root, "dist/downloads/fcsc");
const siteDownload = resolve(root, "site/public/downloads/fcsc-linux-x86_64");

await mkdir(resolve(root, "dist/downloads"), { recursive: true });
await mkdir(resolve(root, "site/public/downloads"), { recursive: true });
await Promise.all([copyFile(binary, artifact), copyFile(binary, siteDownload)]);
