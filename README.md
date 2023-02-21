# image2binary
Converts PNG files to binary data for X16 VERA usage.

This document is for version V1.5 of the program.

V1.0 - initial upload<br>
V1.1 - output file path fix<br>
V1.2 - include 2 zero bytes in front of binary image data<br>
V1.3 - support creating VRAM memory map<br>
V1.4 - output 2 extra files when image crosses VRAM page boundary<br>
V1.5 - support 1, 2, 4, and 8 bits per pixel in output, plus specifying palette offsets<br>

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
with different purposes (such as tiles versus sprites) into a VERA memory map,
because you can tell the program how to align the memory areas (meaning what
alignment boundaries to use).

It may not be needed, but just for convenience sake,
if any binary output file would cross the VRAM page boundary, the program outputs
two extra binary files, so that the data bytes can be loaded in two smaller sections (rather than one large whole),
one section into the last portion of VRAM page #0,
and the other section into the first portion of VRAM page #1. The ROM load
function apparently supports crossing that boundary, so this particular feature may
not be very useful.

Note: This program does not recursively traverse directories. To process subdirectories,
run the program multiple times, with different command line arguments.

The command-line format for this program is as follows:

```
image2binary { [-w width] [-h height] [-b <1|2|4|8>] [-p offset] [-n] [-a alignment] [ <dir2|png2> | ./] } ...
```

'-w' and '-width' are synonyms (either one is allowed)<br>
For a PNG file (in a directory or specific), 'width' is given in pixels.
For a tile map base (see 'a', below), 'width' is the number of horizontal tiles per row, which
equals the total number of columns.<br>
<br>
'-h' and '-height' are synonyms<br>
For a PNG file (in a directory or specific), 'height' is given in pixels.
For a tile map base (see 'a', below), 'height' is the number of vertical tiles per column, which equals the total number of rows.<br>
<br>
'b' and '-bpp' are synonyms<br>
This may be used to specify the number of bits per pixel in the output binary
file, which provides the intended range of color indexes (1: 2 colors, 2: 4
colors, 4: 16 colors, 8: 256 colors). The default value is 8, for 256 colors.
As always, color index #0 means transparent, so the actual number of colors is
one less than the range might imply.<br>
<br>
'-p' and '-paletteoffset' are synonymns<br>
If the specified number of bits per pixel ('-b' or '-bpp') is less than 8, then
you may specify which palette offset to use when placing the colors for the
image into the palette. The 'offset' must be a number from 1 to 15. Note that
if the number of bits per pixel is 1 (for 2 colors) or 2 (for 4 colors), then
the remaining palette (14 or 12) colors within the designated 16-color palette section may be used for colors of other images, as needed. If you do not specify
the palette offset, one will be chosen automatically.<br>
<br>
If you use the same palette offset index (i.e., share it)
for multiple input files, be sure to list
the files in order of their bits-per-pixel numbers, from lowest to highest. For example,
if a 2-bpp file and a 4-bpp file share the same palette offset, list the 2-bpp file
before the 4-bpp file, in the command line.<br>
<br>
'-n' and '-nooutput' are synonymns<br>
When this option is specified, the output file will not exist, meaning that there will
be no output file for the given input image. This option may be used simply to modify
the color palette, and is particularly useful when using a single output image that
may be shown in several different base colors, using different palette offsets. In that
case, multiple input images should be used, one of which does <i>not</i> use the 'no output'
option, and all such images should be identical in appearance, except for their colors.
The expectation is that the number of unique colors,
and the order in which pixels of corresponding colors are encountered,
within each file, is the same.<br>
<br>
Using 'no output' while also sharing palette offsets (see '-p', above) might result in
improper colors being displayed, so be careful about choosing palette offsets!<br>
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
./image2binary -w 320 -h 240 painting.png >painting.log
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
../image2binary \
 -w 64 -h 32 -a mb alpha-tile-map \
 -w 64 -h 32 -a mb image-tile-map \
 -a tb abctiles.png \
 -a tb brdtiles.png \
 -b 1 -p 15 monochrome.png \
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
    0 $00000 $00fff  4096   512    64    32  alpha-tile-map
    0 $01000 $01fff  4096   512    64    32  image-tile-map
    0 $02000 $0a2ff 33536  2048    16  2096  brdtiles.png
    0 $0a300 $0aeff  3072    32    16   192  seq16.png
    0 $0af00 $0af19    26     1    13    13  monochrome.png
  230 $0b000 $0feff 20224  2048    16  1264  abctiles.png
    0 $0ff00 $1beff 49152    32    64   768  seq64.png
      $0ff00 $0ffff   256                    SEQ64P0.BIN
      $10000 $1beff 48896                    SEQ64P1.BIN
    0 $1bf00 $1eeff 12288    32    32   384  seq32.png
    0 $1ef00 $1f1ff   768    32     8    96  seq08.png

NOTE: one output image crosses the VRAM page boundary, so there are now two
      extra output files, for loading the data in two sections, if needed.
```

In the above example, there is one file that is processed with 1 bit-per-pixel color.
The PNG file contains pixels that are either white or transparent (there is no black).
Also, the file is designated to use palette offset 15, meaning the last portion of
the palette. The resulting palette contains the following lines in it. This implies
that any '0' bits in the MONOCHROME.PNG file will show as transparent, and any '1'
bits in that file will show as white (color index #1 is RGB(15,15,15)), in the palette.

```
    ...
    .byte    $00,$00  ; 239 $ef:  0 0 0 (FREE)
    .byte    $00,$00  ; 240 $f0:  0 0 0 (FREE)
    .byte    $ff,$0f  ; 241 $f1:  f f f
    .byte    $00,$00  ; 242 $f2:  0 0 0 (FREE)
    ...
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
