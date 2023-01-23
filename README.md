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

The palette contents, <i>without the leading 2-byte address offset,</i>
are printed to the console (and can be piped to a file), as ca65 assembler source text.
The palette contents, <i>with the leading 2-byte address offset,</i>
are written to the "PALETTE.BIN" file, and can be loaded into the VERA using the VLOAD
command in BASIC.

The output bitmap (set of palette indexes representing colors) can also be loaded
into VRAM using the VLOAD command. That file does not have any leading data bytes,
only pixel information.

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
```

As an example, the "painting.png" file in the "samples"" directory of this project was
processed using "-w 320 -h 240" as the command parameters (note the spaces), to yield the BIN file in that same directory. Here is the entire command line:

```
./image2binary -w 320 -h 240 >painting.log
```

The image can be displayed using the following steps:

* Change to the "samples" directory.
* Run the X16 emulator. You may need to specify the path, unless it is reachable.
* After BASIC loads to its initial screen, load and run "PAINTING.BAS".

There are also other example conversions of the same file, as shell scripts, in sub-directories off of the "samples" directory. If you run a script,
it should convert the image file to binary, based on the command line in
the script. Some of the output sizes generated by these sample scripts are
physically too large to fit inside the VRAM, but using the output is not
the point; these are just examples to show how the command line affects
the output data.

On a <i>per-directory</i> basis, you may choose to specify the output width and/or height, in pixels.
If neither width nor height is specified, then the width and height are taken from
the input files. If one or both dimensions are specified, then the output pixel data
(palette map indexes) is sized accordingly, either by padding with transparent pixels,
or by cropping (discarding) extra pixels. The input is always centered over the output.

For example, using an input image of 57x64 pixels (width x height), and a command
line option "-w 64", the output image will be 64x64 pixels, because the height is
taken from the input image file. Specifying "-w 640 -h 480" for the same input image
will result in the original, small image being centered in a 640x480 space.

NOTE: This program does <b>not</b> resize an image by stretching or shrinking it, and it does <b>not</b> attempt to optimize the palette. The only color
conversion that is does is to take 24-bit RGB data, and right-shift each of
the color components by 2 (i.e., divide by 4), to yield 12-bit color.
This implies that detail may be lost, if the original image had non-zero
values in the least significant 2 bits of any color component of any pixel.

The overall processing is as follows:
* Obtain a list of all files to process.
* Read all files.
* Determine how many unique 12-bit colors are used in ALL files together.
* Organize a new color palette (index 0 means transparent; indexes 1..255 mean color).
* Output palette data as assembler source text.
* Output image data as binary palette indexes, one index per pixel.

NOTE: Regardless of which portion (some or all) of each input file is copied
(either in whole or in part) to the output, the <b>entire</b> input image is used to determine the combined palette. The main intent of this program is
to create a single palette that can be used for multiple images, tiles,
and/or sprites, so that they can all be shown on a single screen.
