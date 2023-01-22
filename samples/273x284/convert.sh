#!/bin/sh

# Should produce a 273x284 binary image (10000 bytes).
../image2binary >convert.log
cat convert.log

