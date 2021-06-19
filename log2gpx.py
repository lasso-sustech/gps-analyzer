#!/usr/bin/env python3
import re
from pathlib import Path
from halo import Halo
import logging, datetime
import pynmea2
from pynmea2.types.talker import ZDA, GGA
import lxml.etree as ET

GPX_DIR = Path('./gpx')
FILTER=re.compile('(\$GPZDA,).*|(\$GPGGA,).*')
logger = logging.getLogger()

log_files = Path('./logs').glob('*.log')
log_files = sorted(log_files)

for _file in log_files:
    _name = _file.stem
    with Halo('Generate %s.gpx'%_name) as sh:
        #
        root = ET.Element('gpx', version='1.0')
        track= ET.SubElement(root, 'trk')
        ET.SubElement(track, 'name').text   = _name
        ET.SubElement(track, 'number').text = '1' #one track segment
        trkseg = ET.SubElement(track, 'trkseg')
        # load and parse log data
        with open(_file) as fh:
            last_time = None
            last_qual = None
            for idx,line in enumerate(fh.readlines()):
                if not FILTER.match(line):
                    break #EOF
                _obj = pynmea2.parse(line)
                if isinstance(_obj, ZDA):
                    last_time = _obj.datetime.isoformat()
                elif isinstance(_obj, GGA):
                    _lat, _lon, _ele = _obj.latitude, _obj.longitude, _obj.altitude
                    if _lat is None or _lon is None or _ele is None:
                        logger.warning( 'WARNING: GPS loss at %s (line %d)'%(last_time, idx) )
                        last_qual = 0
                    else:
                        if _ele<20 or _ele>50:
                            logger.warning( 'WARNING: Altitude Value Error at %s (line %d)'%(last_time, idx) )
                            _ele = 0
                        # insert trackpoint
                        _trkpt = ET.SubElement(trkseg, 'trkpt', lat=str(_lat), lon=str(_lon))
                        ET.SubElement(_trkpt, 'ele').text  = str(_ele)
                        ET.SubElement(_trkpt, 'time').text = str(last_time)
                        # insert waypoint
                        this_qual = _obj.gps_qual
                        if (this_qual!=last_qual) and (this_qual==4 or last_qual==4):
                            _wpt = ET.SubElement(root, 'wpt', lat=str(_lat), lon=str(_lon))
                            ET.SubElement(_wpt, 'ele').text = str(_ele)
                            ET.SubElement(_wpt, 'name').text= 'Weak-%s-%s'%(
                                'stop' if this_qual==4 else 'start',
                                last_time
                                )
                            pass
                        last_qual = this_qual
                    last_time = None
                    pass
            pass
        # write to gpx file
        root_tree = ET.ElementTree(root)
        root_tree.write( '%s.gpx'%(GPX_DIR/_name), pretty_print=True )
        sh.succeed()
    pass
