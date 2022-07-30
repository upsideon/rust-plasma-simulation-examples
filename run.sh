#!/bin/bash

docker build -t plasma .
docker run -it --rm -v $(pwd):/usr/src/plasma-simulation --name plasma plasma
