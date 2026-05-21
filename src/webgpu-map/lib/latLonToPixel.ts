// https://github.com/SheepTester/ucsd-classrooms/blob/main/src/lib/locations.ts
export const latToY = (lat: number) =>
  -(Math.log(Math.tan(Math.PI / 4 + (lat * Math.PI) / 360)) / Math.PI + 1);
export const lonToX = (lon: number) => lon / 180 + 1;
