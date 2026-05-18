const server = Bun.serve({
  port: 3000,
  routes: {
    "/": () => new Response(Bun.file("./src/index.html"), {
      headers: { "Content-Type": "text/html" }
    }),
    "/main.js": async () => {
      const build = await Bun.build({
        entrypoints: ["./src/main.ts"],
        minify: true,
      });
      return new Response(build.outputs[0], {
        headers: { "Content-Type": "text/javascript" }
      });
    },
    "/css/maplibre-gl.css": () => new Response(Bun.file("./node_modules/maplibre-gl/dist/maplibre-gl.css"), {
      headers: { "Content-Type": "text/css" }
    }),
    "/css/compass.css": () => new Response(Bun.file("./node_modules/@mapbox-controls/compass/src/index.css"), {
      headers: { "Content-Type": "text/css" }
    }),
    "/css/ruler.css": () => new Response(Bun.file("./node_modules/@mapbox-controls/ruler/src/index.css"), {
      headers: { "Content-Type": "text/css" }
    }),
  }
});

console.log(`Listening on ${server.url}`);
