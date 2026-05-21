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
    const extremes = i % 2 === 0 ? lat : lon;
    nodeCoords[i] =
      extremes.min +
      (view.getUint16(36 + i * 2) / 0xffff) * (extremes.max - extremes.min);
  }
  const wayIndicesStart = 36 + nodeCount * 4;
  const wayIndices = new Uint32Array((bin.byteLength - wayIndicesStart) / 4);
  for (let i = 0; i < wayIndices.length; i++) {
    wayIndices[i] = view.getUint32(wayIndicesStart + i * 4);
  }
  console.timeEnd("parseStreets");
  return { nodeCoords, wayIndices };
}
