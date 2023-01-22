#!/bin/sh

# Should produce a 100x100 binary image (10000 bytes).
../image2binary -w 100 -h 100 >convert.log
cat convert.log


