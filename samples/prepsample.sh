# To be run from 'samples' directory.
clear
cd ..
cargo build --release
cd samples
rm PAINTING.BIN
rm PALETTE.BIN
rm painting.log
rm image2binary
ls -l
cp ../target/release/image2binary ./
./image2binary -w 320 -h 240 painting.png >painting.log
ls -l
hexdump -C PALETTE.BIN
gedit painting.log &

cd alignment
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
gedit alignment.log &
cd ..

