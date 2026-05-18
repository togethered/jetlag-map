import maplibregl from 'maplibre-gl';

const map = new maplibregl.Map({
    container: 'map', // container id
    style: 'https://demotiles.maplibre.org/globe.json', // style URL
    center: [37.452520, 122.265640], // starting position [lng, lat]
    zoom: 1 // starting zoom
});
