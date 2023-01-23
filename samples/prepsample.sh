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
./image2binary -w 320 -h 240 >painting.log
ls -l
hexdump -C PALETTE.BIN
gedit painting.log &

