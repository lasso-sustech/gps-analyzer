#!/usr/bin/env python3
from sys import argv
from pathlib import Path
from gpxplotter import read_gpx_file, create_folium_map, add_segment_to_map

# Define some properties for drawing the line:
line_options = {'color': 'red', 'weight': 8, 'opacity': 0.5}

if len(argv) < 2:
    print('Usage: `plotgpx <gpx_file_path>`')
    exit()

the_file = argv[1]
the_name = Path(the_file).stem
the_map = create_folium_map()
for track in read_gpx_file(the_file):
    for i, segment in enumerate(track['segments']):
        add_segment_to_map(the_map, segment, line_options=line_options)
    
the_map.save( 'trajectory-%s.html'%the_name )