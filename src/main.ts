import maplibregl from 'maplibre-gl';
import CompassControl from '@mapbox-controls/compass';
import RulerControl from '@mapbox-controls/ruler';

const map = new maplibregl.Map({
    container: 'map',
    style: 'https://api.maptiler.com/maps/streets-v4/style.json?key=4F7xYJRpkPdD3qSM9UoZ',
    center: [-122.420679, 37.772537], // San Francisco
    zoom: 12
});

// Add Compass Control
map.addControl(new CompassControl() as any, 'top-right');

// Add Ruler Control
map.addControl(new RulerControl() as any, 'bottom-right');

// Ruler Events
map.on('ruler.on', () => console.log('Ruler activated'));
map.on('ruler.off', () => console.log('Ruler deactivated'));

