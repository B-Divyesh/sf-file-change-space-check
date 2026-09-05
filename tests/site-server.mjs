import { createServer } from "node:http";
import { access, readFile } from "node:fs/promises";
import { extname, resolve, sep } from "node:path";
import { spawnSync } from "node:child_process";

const repository = resolve(import.meta.dirname, "..");
const root = resolve(repository, "dist/site");
const types = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".jpg": "image/jpeg",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".txt": "text/plain; charset=utf-8",
  ".webp": "image/webp",
  ".xml": "application/xml; charset=utf-8",
};

export async function ensureBuild() {
  try {
    await access(resolve(root, "index.html"));
    await access(resolve(repository, "dist/downloads/fcsc"));
  } catch {
    const result = spawnSync("npm", ["run", "build"], { cwd: repository, encoding: "utf8" });
    if (result.status !== 0) {
      throw new Error(`npm run build failed\n${result.stdout}\n${result.stderr}`);
    }
  }
}

export async function startSiteServer() {
  await ensureBuild();
  const server = createServer(async (request, response) => {
    const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://localhost").pathname);
    const relative = pathname === "/"
      ? "index.html"
      : pathname.endsWith("/")
        ? `${pathname.slice(1)}index.html`
        : pathname.slice(1);
    const target = resolve(root, relative);
    const safe = target === root || target.startsWith(`${root}${sep}`);
    try {
      if (!safe) throw new Error("path escapes site root");
      const body = await readFile(target);
      response.writeHead(200, {
        "content-type": types[extname(target)] ?? "application/octet-stream",
        "content-security-policy": "default-src 'self'; img-src 'self'; style-src 'self'; script-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'; form-action 'self'",
        "referrer-policy": "no-referrer",
        "x-content-type-options": "nosniff",
      });
      response.end(body);
    } catch {
      const body = await readFile(resolve(root, "404.html"));
      response.writeHead(404, { "content-type": "text/html; charset=utf-8" });
      response.end(body);
    }
  });
  await new Promise((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("Could not start site server");
  return {
    origin: `http://127.0.0.1:${address.port}`,
    close: () => new Promise((resolveClose, reject) => server.close((error) => error ? reject(error) : resolveClose())),
  };
}

export const paths = { repository, root, binary: resolve(repository, "dist/downloads/fcsc") };
