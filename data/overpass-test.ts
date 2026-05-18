// Usage: bun run data/overpass-test.ts

// https://wiki.openstreetmap.org/wiki/Overpass_API
const OVERPASS_API_URL = "https://overpass-api.de/api/interpreter";

type OverpassResponse = {
  elements: OverpassElement;
};
type OverpassElement = {
  type: "relation";
};

function query(query: string) {
  return fetch(OVERPASS_API_URL, {
    method: "POST",
    body: new URLSearchParams({ data: query }),
    headers: { "user-agent": "https://github.com/togethered/jetlag-map" },
  }).then(async (r) =>
    r.ok
      ? r.json().then((json) => json as OverpassResponse)
      : Promise.reject(
          new Error(
            `HTTP error ${r.status}: ${await r
              .text()
              .then((text) => text.slice(0, 1000))
              .catch(() => r.url)}`
          )
        )
  );
}

const sanFrancisco = [
  // southwest (approximate)
  37.70765015159924, -122.5189634289361,
  // northeast (approximate, includes southern half of treasure island)
  37.816301033516325, -122.35772673478483,
].join(",");

Bun.write(
  "data/train-lines.json",
  JSON.stringify(
    await query(`
      [bbox:${sanFrancisco}]
      [out:json]
      ;
      (
        relation["type"="route"]["route"~"train|subway|light_rail"];
      );
      out geom;
    `)
  )
);
console.error("wrote data/train-lines.json");

Bun.write(
  "data/train-stations.json",
  JSON.stringify(
    await query(`
      [bbox:${sanFrancisco}]
      [out:json]
      ;
      (
        node["railway"="station"];
        node["railway"="tram_stop"];
        node["station"="subway"];
        node["public_transport"="station"];
      );
      out body;
    `)
  )
);
console.error("wrote data/train-stations.json");

Bun.write(
  "data/poi.json",
  JSON.stringify(
    await query(`
      [bbox:${sanFrancisco}]
      [out:json]
      ;
      (
        node["tourism"="zoo"];
        way["tourism"="zoo"];
        node["tourism"="aquarium"];
        way["tourism"="aquarium"];
        node["amenity"="library"];
        way["amenity"="library"];
        node["tourism"="museum"];
        way["tourism"="museum"];
        node["amenity"="hospital"];
        way["amenity"="hospital"];
      );
      out center;
    `)
  )
);
console.error("wrote data/poi.json");
