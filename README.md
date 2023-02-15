# image2binary
Converts PNG files to binary data for X16 VERA usage.

This program converts PNG file data into binary data for use on the
Commander X16. It reads multiple PNG files, combines their needed
color palettes, and outputs palette entries in both text and binary, plus it outputs binary pixel data (palette indexes rather than colors). Additionally,
the program arranges a VRAM memory map, and outputs that information,
which may be helpful in loading the binary data into VRAM.

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
into VRAM using the VLOAD command. That file <i>does</i> have the leading dummy address bytes (2 zero bytes).

The program can be run in one of 3 ways. If no directory is specified, or if
the current directory is specified, the app searches for PNG files in the
current directory.

If a list of one or more directories is specified, the app
searches for files in those directories. In either case, all files are
processed together, where the resulting palette is concerned, so that any of
the images can be display on the X16, using the resulting palette.

The third way is that you can specify individual PNG files, instead of or along
with other directories. This can be quite useful when trying to arrange files
with different purposes (such as tiles versus sprites) into a VERA memory map.

This program does not recursively traverse directories. To process subdirectories,
run the program multiple times, with different command line arguments.

The command-line format for this program is as follows:

```
image2binary [-w width] [-h height] [-a alignment] [./]
image2binary [-w width] [-h height] [-a alignment] <dir1|png1> [ { [-w width] [-h height] [<dir2|png2> | ./] } ...]
```

'-w' and '-width' are synonyms (either one is allowed)<br>
For a PNG file (in a directory or specific), 'width' is given in pixels.
For a tile map base (see 'a', below), 'width' is the number of horizontal tiles per row, which
equals the total number of columns.<br>
<br>
'-h' and '-height' are synonyms<br>
For a PNG file (in a directory or specific), 'height' is given in pixels.
For a tile map base (see 'a', below), 'height' is the number of vertical tiles per column, which
equals the total number of rows.<br>
<br>
'-a' and '-alignment' are synonyms<br>
By default, the output binary data from a PNG file is aligned to 1 byte, in the VERA memory map.
Specifying an alignment value (number) causes the output data to be aligned
according to the given number. It would be more typical not to specify a
number, but to use these special values:<br>
<br>
* 'tb': alignment is 2048
* 'tilebase': alignment is 2048
* 'mb': alignment is 512
* 'mapbase': alignment is 512
* 'sp': alignment is 32
* 'sprite': alignment is 32
* 'bm': alignment is 2048
* 'bitmap': alignment is 2048
<br>
<br>
For a tilebase, the path name is treated as a required comment, so it need not contain a valid path, but
if it has special characters or spaces, it may
need to be quoted in the command line.
<br>
<br>
'dir1' and 'dir2' are names or paths of directories<br>
<br>
'png1' and 'png2' are names or paths of individual PNG files<br>
<br>

As an example of changing image size, the "painting.png" file in the "samples"" directory of this project was
processed using "-w 320 -h 240" as the command parameters (note the spaces), to yield the BIN file in that same directory. Here is the entire command line:

```
./image2binary -w 320 -h 240 >painting.log
```

The image can be displayed using the following steps:

* Change to the "samples" directory.
* Run the X16 emulator. You may need to specify the path, unless it is reachable.
* After BASIC loads to its initial screen, load and run "PAINTING.BAS".

Another example illustrates specifying individual files, rather than directories,
and indicating what memory alignment to use for each file. (It is possible to
specify alignment when using a directory, but that causes all files in the
directory to be aligned in the same way.)

The "alignment" sample directory may be processed like this:

```
.../image2binary \
 -w 64 -h 32 -a mb alpha-tile-map \
 -w 64 -h 32 -a mb image-tile-map \
 -a tb abctiles.png \
 -a tb brdtiles.png \
 -a sp seq08.png \
 -a sp seq16.png \
 -a sp seq32.png \
 -a sp seq64.png  >alignment.log
```

The resulting log file will contain a VRAM memory map, such as the following.
The program attempts to arrange memory with the least possible amount of waste.

```
VRAM Address Arrangement

Waste Start  End    Size  Align Width Height Path/Name
----- ------ ------ ----- ----- ----- ------ ----------------------------------
    0 $00000 $081ff 33280  2048    16  2080  brdtiles.png
    0 $08200 $08dff  3072    32    16   192  seq16.png
  512 $09000 $0ddff 19968  2048    16  1248  abctiles.png
    0 $0de00 $0edff  4096   512    64    32  alpha-tile-map
    0 $0ee00 $0fdff  4096   512    64    32  image-tile-map
    0 $0fe00 $1bdff 49152    32    64   768  seq64.png
    0 $1be00 $1edff 12288    32    32   384  seq32.png
    0 $1ee00 $1f0ff   768    32     8    96  seq08.png
```

There are also other example conversions of the 'painting' file, as shell scripts, in sub-directories off of the "samples" directory. If you run a script,
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
the color components by 4 (i.e., divide by 16), to yield 12-bit color from the input 24-bit color.
This implies that detail may be lost, if the original image had non-zero
values in the least significant 4 bits of any color component of any pixel.

The overall processing is as follows:
* Obtain a list of all files to process.
* Read all files.
* Determine how many unique 12-bit colors are used in ALL files together.
* Organize a new color palette (index 0 means transparent; indexes 1..255 mean color).
* Output palette information as binary data.
* Output palette information as assembler source text.
* Output image data as binary palette indexes, one index per pixel.
* Compute and output VERA memory map as text.

NOTE: Regardless of which portion (some or all) of each input file is copied
(either in whole or in part) to the output, the <b>entire</b> input image is used to determine the combined palette. The main intent of this program is
to create a single palette that can be used for multiple images, tiles,
and/or sprites, so that they can all be shown on a single screen.
