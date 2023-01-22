# image2binary
Converts PNG files to binary data for X16 VERA usage.

This program converts PNG file data into binary data for use on the
Commander X16. It reads multiple PNG files, combines their needed
color palettes, and outputs palette entries as text, plus binary
pixel data (palette indexes rather than colors) files.

NOTE: It assumes using 8 bits-per-pixel in the <i>output</i> for VERA; however, the input PNG files may contain 24-bit RGB or 32-bit RGBA data.

The output palette will always be a set of 255 (or less) 12-bit colors,
meaning that it represents (up to) 255 colors out of a set of 4096 possible
colors. Color index 0 always represents transparency.
Color indexes 1-15 are reserved for standard palette entries, leaving
up to 240 custom color indexes available.
Any trailing unused palette indexes may be used at your discretion.

The program can be run in one of 2 ways. If no directory is specified, or if
the current directory is specified, the app searches for PNG files in the
current directory. If a list of one or more directories is specified, the app
searches for files in those directories. In either case, all files are
processed together, where the resulting palette is concerned, so that any of
the images can be display on the X16, using the resulting palette.

This program does not recursively traverse directories. To process subdirectories,
run the program multiple times, with different command line arguments.

```
image2binary [-w width] [-h height] [./]
image2binary [-w width] [-h height] dir1 [ { [-w width] [-h height] [dir2 | ./] } ...]

Examples:

```

On a <i>per-directory</i> basis, you may choose to specify the output width and/or height, in pixels.
If neither width nor height is specified, then the width and height are taken from
the input files. If one or both dimensions are specified, then the output pixel data
(palette map indexes) is sized accordingly, either by padding with transparent pixels,
or by cropping (discarding) extra pixels. The input is always centered over the output.

NOTE: This program does <b>not</b> resize an image by stretching or shrinking it.

For example, using an input image of 57x64 pixels (width x height), and a command
line option "-w 64", the output image will be 64x64 pixels, because the height is
taken from the input image file. Specifying "-w 640 -h 480" for the same input image
will result in the original, small image being centered in a 640x480 space.

The overall processing is as follows:
* Obtain a list of all files to process.
* Read all files.
* Determine how many unique 12-bit colors are used in ALL files together.
* Organize a new color palette (index 0 means transparent; indexes 1..255 mean color).
* Output palette data as assembler source text.
* Output image data as binary palette indexes, one index per pixel.
