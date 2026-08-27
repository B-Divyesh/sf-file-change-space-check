import { copyFile, mkdir } from "node:fs/promises";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const binary = resolve(root, "target/release/fcsc");
const artifact = resolve(root, "dist/downloads/fcsc");

await mkdir(resolve(root, "dist/downloads"), { recursive: true });
await copyFile(binary, artifact);
