#!/usr/bin/env python3
from sys import argv
from pathlib import Path

if len(argv) < 2:
    print('Usage: `plotgpx <gpx_file_path>`')
    exit()

the_file = argv[1]
the_name = Path(the_file).stem

## Using gpxplotter
# from gpxplotter import read_gpx_file, create_folium_map, add_segment_to_map
# the_map = create_folium_map()
# # Define some properties for drawing the line:
# line_options = {'color': 'red', 'weight': 8, 'opacity': 0.5}

# for track in read_gpx_file(the_file):
#     for i, segment in enumerate(track['segments']):
#         add_segment_to_map(the_map, segment, line_options=line_options)
# the_map.save( 'trajectory-%s.html'%the_name )


## Using trackanimation
import trackanimation
from trackanimation.animation import AnimationTrack

trk = trackanimation.read_track(the_file)
fig = AnimationTrack(df_points=trk, dpi=300, bg_map=True, map_transparency=0.5)
fig.make_video(output_file=the_name, framerate=60, linewidth=1.0)
