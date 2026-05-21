// https://github.com/SheepTester/ucsd-classrooms/blob/main/src/lib/locations.ts
export const latToY = (lat: number) =>
  -(Math.log(Math.tan(Math.PI / 4 + (lat * Math.PI) / 360)) / Math.PI + 1);
export const lonToX = (lon: number) => lon / 180 + 1;

/**
 * in miles, because hide + seek is designed for miles
 *
 * https://en.wikipedia.org/wiki/Earth_radius#Geocentric_radius
 * a = 6378137.0
 * b = 6356752.3
 * lat = 37.75 / 180 * Math.PI
 *
 * Math.sqrt(((a * a * Math.cos(lat)) ** 2 + (b * b * Math.sin(lat)) ** 2) / ((a * Math.cos(lat)) ** 2 + (b * Math.sin(lat)) ** 2))
 * 6370163.827537604 meters
 * 3958.23629 miles
 */
const EARTH_RADIUS = 3958.23629;
/**
 * haversine formula (untested), if we want precise circles on the map
 *
 * lat/lon needs to be in radians
 */
export const dist = (
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number,
) => {
  const a =
    Math.sin((lat2 - lat1) / 2) ** 2 +
    Math.cos(lat1) * Math.cos(lat2) * Math.sin((lon2 - lon1) / 2);
  return 2 * EARTH_RADIUS * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
};
