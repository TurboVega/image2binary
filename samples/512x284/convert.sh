#!/bin/sh

# Should produce a 512x284 binary image (145408 bytes).
../image2binary -w 512 >convert.log
cat convert.log

