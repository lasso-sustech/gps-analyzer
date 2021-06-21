
# GPS Analyzer for Raw Data

### Prerequisite
Install the runtime dependency

```bash
    pip3 install -r requirements
```

Install the video plot dependency
```bash
    pip3 install ./trackanimation-1.0.5-py2.py3-none-any.whl
    sudo apt install ffmpeg
```

### Convert Raw GSP Data

1. put raw data files with extension `*.log` in `./logs` folder;

2. run `python3 ./log2gpx.py`, and the `*.gpx` files are generated correspondingly in `./gpx` folder.
> Run `python3 ./log2gpx.py 2>/dev/null` to disable the WARNING message

### Plot GSP Track
```bash
    python3 ./plotgpx.py ./gpx/<gpx_file>
```
