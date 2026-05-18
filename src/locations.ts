export interface POI {
    name: string;
    category: string;
    coordinates: [number, number];
}

export const pois: POI[] = [
    { name: "The Castro Theatre", category: "Movie Theater", coordinates: [-122.4347, 37.7620] },
    { name: "Alamo Drafthouse Cinema", category: "Movie Theater", coordinates: [-122.4182, 37.7554] },
    { name: "Roxie Theatre", category: "Movie Theater", coordinates: [-122.4202, 37.7648] },
    { name: "Aquarium of the Bay", category: "Aquarium", coordinates: [-122.4103, 37.8083] },
    { name: "Steinhart Aquarium", category: "Aquarium", coordinates: [-122.4661, 37.7699] },
    { name: "SFMOMA", category: "Museum", coordinates: [-122.4011, 37.7857] },
    { name: "de Young Museum", category: "Museum", coordinates: [-122.4687, 37.7715] },
    { name: "The Exploratorium", category: "Museum", coordinates: [-122.3975, 37.8014] },
    { name: "Legion of Honor", category: "Museum", coordinates: [-122.5008, 37.7845] },
];
