import { build } from "vite";
import { copyFile, mkdir, readdir, stat, writeFile } from "node:fs/promises";
import { resolve, relative, sep } from "node:path";

const root = resolve(import.meta.dirname, "..");
const output = resolve(root, "dist/site");
await build({ configFile: resolve(root, "site/vite.config.ts") });

const cliArtifact = resolve(root, "dist/downloads/fcsc");
try {
  await stat(cliArtifact);
  await mkdir(resolve(output, "downloads"), { recursive: true });
  await copyFile(cliArtifact, resolve(output, "downloads/fcsc-linux-x86_64"));
} catch (error) {
  if (error.code !== "ENOENT") throw error;
}

async function filesBelow(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = await Promise.all(entries.map((entry) => {
    const path = resolve(directory, entry.name);
    return entry.isDirectory() ? filesBelow(path) : [path];
  }));
  return files.flat();
}

const precacheFiles = (await filesBelow(output))
  .filter((path) => /\.(?:html|css|js|svg|webp|jpg|png|txt|xml)$/.test(path) && !path.endsWith(`${sep}sw.js`))
  .map((path) => `/${relative(output, path).split(sep).join("/")}`)
  .sort();
const precache = [...new Set(precacheFiles.flatMap((path) => {
  if (path === "/index.html") return [path, "/"];
  if (path.endsWith("/index.html")) return [path, path.slice(0, -"index.html".length)];
  return [path];
}))];

const serviceWorker = `const CACHE = "fcsc-shell-v2";
const SHELL = ${JSON.stringify(precache)};
self.addEventListener("install", event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(SHELL)));
  self.skipWaiting();
});
self.addEventListener("activate", event => {
  event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key)))));
  self.clients.claim();
});
self.addEventListener("fetch", event => {
  if (event.request.method !== "GET") return;
  event.respondWith(caches.match(event.request).then(cached => cached ?? fetch(event.request).then(response => {
    if (response.ok) caches.open(CACHE).then(cache => cache.put(event.request, response.clone()));
    return response;
  }).catch(() => caches.match("/index.html"))));
});
`;
await writeFile(resolve(output, "sw.js"), serviceWorker);
