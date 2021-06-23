#!/usr/bin/env python3
from sys import argv
from pathlib import Path
import lxml.etree as ET
from datetime import datetime, timedelta
import random

if len(argv) < 2:
    print('Usage: `gpx-obfuscation <gpx_file_path>`')
    exit()

the_file   = argv[1]
the_folder = Path(the_file).parent
the_name   = Path(the_file).stem

tree = ET.parse(the_file)
root = tree.getroot()

# <trkpt lat="22.603808605" lon="113.99878587833334">
#     <ele>47.8572</ele>
#     <time>2021-06-19T09:18:43.900000-08:00</time>
# </trkpt>

_accept = False
while not _accept:
    lat_offset  = random.randint(10, 50); random.seed( lat_offset ); lat_offset += random.random()
    lon_offset  = random.randint(20, 100); random.seed( lon_offset ); lon_offset += random.random()
    time_offset = timedelta( days=random.randint(1,15), hours=random.randint(0, 24) )
    print(lat_offset, lon_offset, time_offset)
    _accept = input('Ok (y/N)?') in ['y', 'Y']

for pt in root.iter('trkpt'):
    _lat, _lon = pt.get('lat'), pt.get('lon')
    _ele, _time = pt[0], pt[1]
    #
    obs_lat = float(_lat) - lat_offset
    obs_lon = float(_lon) - lon_offset
    obs_ele = 0
    obs_time = _time.text[ :_time.text.rfind('-') ]
    obs_time = datetime.fromisoformat(obs_time) - time_offset
    #
    pt.attrib['lat'] = '%.8f'%obs_lat
    pt.attrib['lon'] = '%.8f'%obs_lon
    _ele.text  = '0'
    _time.text = obs_time.isoformat()
    pass

tree = ET.ElementTree(root)
tree.write( '%s-obs.gpx'%(the_folder/the_name), pretty_print=True )
