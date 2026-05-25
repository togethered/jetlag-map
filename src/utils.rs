use std::f64::consts::PI;

// https://github.com/SheepTester/ucsd-classrooms/blob/main/src/lib/locations.ts
// According to Gemini, this flattens the globe into a [-1, 1] square
// "web mercator"
pub fn lat_to_y(lat: f64) -> f64 {
    -(lat / 2.0 + 45.0).to_radians().tan().ln() / PI
}
pub fn lon_to_x(lon: f64) -> f64 {
    lon / 180.0
}
