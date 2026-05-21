import { join } from "node:path";

type ReturnsFile = (isBuild: boolean) => Blob | "" | Promise<Blob | "">;

const files: Record<string, ReturnsFile> = {
  "index.html": () => Bun.file("./src/index.html"),
  "main.js": async (isBuild) => {
    const build = await Bun.build({
      entrypoints: ["./src/main.ts"],
      minify: isBuild,
    });
    return build.outputs[0] ?? "";
  },
  "css/maplibre-gl.css": () =>
    Bun.file("./node_modules/maplibre-gl/dist/maplibre-gl.css"),
  "css/compass.css": () =>
    Bun.file("./node_modules/@mapbox-controls/compass/src/index.css"),
  "css/ruler.css": () =>
    Bun.file("./node_modules/@mapbox-controls/ruler/src/index.css"),
  "/css/export.css": () =>
    Bun.file(
      "./node_modules/@watergis/maplibre-gl-export/dist/maplibre-gl-export.css"
    ),
  "webgpu-map.html": () => Bun.file("./src/webgpu-map/index.html"),
  "webgpu-index.js": async (isBuild) => {
    const build = await Bun.build({
      entrypoints: ["./src/webgpu-map/index.ts"],
      minify: isBuild,
    });
    return build.outputs[0] ?? "";
  },
};

const [, , mode = ""] = Bun.argv;
const usage = `usage: bun run index.ts (serve|build)`;
switch (mode) {
  case "":
  case "serve": {
    if (mode === "") {
      console.warn("warn: Defaulting to serve mode.");
      console.warn(usage);
    }
    const server = Bun.serve({
      port: 3000,
      routes: Object.fromEntries(
        Object.entries(files).map(([path, handler]) => [
          path === "index.html" ? "/" : "/" + path,
          async () => {
            return new Response(await handler(false), {
              headers: {
                "Content-Type": path.endsWith(".js")
                  ? "text/javascript"
                  : path.endsWith(".css")
                  ? "text/css"
                  : "text/html",
              },
            });
          },
        ])
      ),
    });
    console.log(`Listening on ${server.url}`);
    break;
  }
  case "build": {
    for (const [path, handler] of Object.entries(files)) {
      await Bun.write(join("dist", path), await handler(true));
    }
    break;
  }
  default: {
    console.error(usage);
    process.exit(1);
  }
}
