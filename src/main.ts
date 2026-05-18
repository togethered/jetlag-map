import maplibregl from 'maplibre-gl';
import CompassControl from '@mapbox-controls/compass';
import RulerControl from '@mapbox-controls/ruler';
import { MaplibreExportControl, Size, PageOrientation, Format, DPI } from '@watergis/maplibre-gl-export';
import { pois } from './locations';

const map = new maplibregl.Map({
    container: 'map',
    style: 'https://api.maptiler.com/maps/streets-v4/style.json?key=4F7xYJRpkPdD3qSM9UoZ',
    center: [-122.420679, 37.772537], // San Francisco
    zoom: 12
});

map.on('load', async () => {
    // Add Compass Control
    map.addControl(new CompassControl() as any, 'top-right');

    // Add Ruler Control
    map.addControl(new RulerControl() as any, 'bottom-right');

    // Add Export Control
    map.addControl(new MaplibreExportControl({
        PageSize: Size.A4,
        PageOrientation: PageOrientation.Portrait,
        Format: Format.PNG,
        DPI: DPI[96],
        Crosshair: true,
        PrintableArea: true,
    }), 'top-right');

    // Load a custom icon
    const image = await map.loadImage('https://upload.wikimedia.org/wikipedia/commons/7/7c/201408_cat.png');
    map.addImage('cat', image.data);

    // Add POI Source
    map.addSource('pois', {
        type: 'geojson',
        data: {
            type: 'FeatureCollection',
            features: pois.map(poi => ({
                type: 'Feature',
                geometry: { type: 'Point', coordinates: poi.coordinates },
                properties: { name: poi.name, category: poi.category }
            }))
        }
    });

    // Add POI Layer
    map.addLayer({
        id: 'poi-layer',
        type: 'symbol',
        source: 'pois',
        layout: {
            'icon-image': 'cat',
            'icon-size': 0.1,
            'text-field': ['get', 'name'],
            'text-variable-anchor': ['top', 'bottom', 'left', 'right'],
            'text-radial-offset': 0.5,
            'text-justify': 'auto',
            'text-size': 12
        },
        paint: {
            'text-color': '#333',
            'text-halo-color': '#fff',
            'text-halo-width': 1
        },
        filter: ['==', ['get', 'category'], 'none'] // Start with nothing visible
    });

    // Handle Filtering
    const radioButtons = document.querySelectorAll('input[name="category"]');
    radioButtons.forEach(radio => {
        radio.addEventListener('change', (e) => {
            const category = (e.target as HTMLInputElement).value;
            if (category === 'none') {
                map.setFilter('poi-layer', ['==', ['get', 'category'], 'none']);
            } else {
                map.setFilter('poi-layer', ['==', ['get', 'category'], category]);
            }
        });
    });

    // Ruler Events
    map.on('ruler.on', () => console.log('Ruler activated'));
    map.on('ruler.off', () => console.log('Ruler deactivated'));
});
