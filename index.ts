import { join } from "node:path";

const files = {
  "index.html": () => Bun.file("./src/index.html"),
  "main.js": async () => {
    const build = await Bun.build({
      entrypoints: ["./src/main.ts"],
      minify: true,
    });
    return build.outputs[0] ?? "";
  },
  "css/maplibre-gl.css": () =>
    Bun.file("./node_modules/maplibre-gl/dist/maplibre-gl.css"),
  "css/compass.css": () =>
    Bun.file("./node_modules/@mapbox-controls/compass/src/index.css"),
  "css/ruler.css": () =>
    Bun.file("./node_modules/@mapbox-controls/ruler/src/index.css"),
};

const [bun, script, mode] = Bun.argv;
switch (mode) {
  case "serve": {
    const server = Bun.serve({
      port: 3000,
      routes: Object.fromEntries(
        Object.entries(files).map(([path, handler]) => [
          path === "index.html" ? "/" : "/" + path,
          async () => {
            return new Response(await handler(), {
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
      await Bun.write(join("dist", path), await handler());
    }
    break;
  }
  default: {
    console.error(`Usage: ${bun} run ${script} (serve|build)`);
    process.exit(1);
  }
}
