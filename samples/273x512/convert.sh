#!/bin/sh

# Should produce a 273x512 binary image (139776 bytes).
../image2binary -h 512 >convert.log
cat convert.log


