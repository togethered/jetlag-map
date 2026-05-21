import { latToY, lonToX } from "./latLonToPixel";

export function parseStreets(bin: ArrayBuffer) {
  // this could probably be negligibly faster had i encoded things in little
  // endian
  console.time("parseStreets");
  const view = new DataView(bin);
  const lat = { min: view.getFloat64(0), max: view.getFloat64(8) };
  const lon = { min: view.getFloat64(16), max: view.getFloat64(24) };
  const nodeCount = view.getUint32(32);
  const nodeCoords = new Float64Array(nodeCount * 2);
  for (let i = 0; i < nodeCoords.length; i++) {
    const isLat = i % 2 === 0;
    const extremes = isLat ? lat : lon;
    nodeCoords[isLat ? i + 1 : i - 1] = (isLat ? latToY : lonToX)(
      extremes.min +
        (view.getUint16(36 + i * 2) / 0xffff) * (extremes.max - extremes.min),
    );
  }
  let i = 36 + nodeCount * 4;
  const wayIndices: Uint32Array[] = [];
  while (i < bin.byteLength) {
    const nodeCount = new Uint32Array(view.getUint16(i));
    wayIndices.push(nodeCount);
    let prev = (nodeCount[0] = view.getInt32(i + 2));
    for (let j = 1; j < nodeCount.length; j++) {
      prev = nodeCount[j] = prev + view.getInt16(i + 6 + (j - 1) * 2);
    }
    i += 6 + (nodeCount.length - 1) * 2;
  }
  if (i !== bin.byteLength) {
    throw new RangeError(`cursor at ${i}, should be at ${bin.byteLength}`);
  }
  console.timeEnd("parseStreets");
  return { nodeCoords, wayIndices };
}
