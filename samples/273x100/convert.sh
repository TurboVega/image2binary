#!/bin/sh

# Should produce a 273x100 binary image (27300 bytes).
../image2binary -h 100 >convert.log
cat convert.log

