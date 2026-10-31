#!/usr/bin/env node
// Serve the working directory over HTTP for the standalone browser harness.
//
// Usage: node scripts/serve-static.mjs [port]   (default 8130)
// Uses only Node's standard library. Directories serve their index.html, or a
// plain listing when they have none.
import { createReadStream, readdirSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, relative, resolve, sep } from "node:path";

const TYPES = {
  ".css": "text/css",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript",
  ".json": "application/json",
  ".md": "text/markdown; charset=utf-8",
  ".mjs": "text/javascript",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".txt": "text/plain; charset=utf-8",
  ".wasm": "application/wasm",
  ".woff2": "font/woff2",
};

const root = resolve(process.cwd());
const port = Number(process.argv[2] ?? 8130);

function escapeHtml(text) {
  return text.replace(/[&<>"]/g, (c) => `&#${c.charCodeAt(0)};`);
}

function send(response, status, body, type = "text/plain; charset=utf-8") {
  response.writeHead(status, { "content-type": type });
  response.end(body);
}

const server = createServer((request, response) => {
  let path;
  try {
    path = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
  } catch {
    return send(response, 400, "Bad request\n");
  }
  const target = resolve(join(root, path));
  // Never serve anything outside the directory the server was started in.
  if (target !== root && !target.startsWith(root + sep)) {
    return send(response, 404, "Not found\n");
  }
  let stats;
  try {
    stats = statSync(target);
  } catch {
    return send(response, 404, "Not found\n");
  }
  if (stats.isDirectory()) {
    if (!path.endsWith("/")) {
      response.writeHead(301, { location: `${path}/` });
      return response.end();
    }
    const index = join(target, "index.html");
    try {
      if (statSync(index).isFile()) {
        response.writeHead(200, { "content-type": TYPES[".html"] });
        return createReadStream(index).pipe(response);
      }
    } catch {
      // No index.html: list the directory instead.
    }
    const title = escapeHtml(`/${relative(root, target).split(sep).join("/")}`);
    const items = readdirSync(target, { withFileTypes: true })
      .map((entry) => {
        const slash = entry.isDirectory() ? "/" : "";
        const href = `${encodeURIComponent(entry.name)}${slash}`;
        return `<li><a href="${href}">${escapeHtml(entry.name + slash)}</a></li>`;
      })
      .join("\n");
    return send(
      response,
      200,
      `<!doctype html>\n<title>${title}</title>\n<h1>${title}</h1>\n<ul>\n${items}\n</ul>\n`,
      TYPES[".html"],
    );
  }
  response.writeHead(200, {
    "content-type": TYPES[extname(target).toLowerCase()] ?? "application/octet-stream",
  });
  createReadStream(target).pipe(response);
});

server.listen(port, () => {
  console.log(`Serving ${root} at http://localhost:${port}/`);
});
