// Please see the README file for an overview of this program.
//
// Copyright 2023 by Curtis Whitley

use std::fs;
use std::io::Write;
use std::env;
use std::collections::HashMap;
use image::{Rgb};

fn main() {
    println!("Image to Binary (file convertor)");

    // Determine which directories to use.
    let mut directories: Vec<String> = vec![];
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        // No command arguments given; use current directory only.
        directories.push("./".to_string());
    } else {
        // Each command argument is a directory name with one or more files.
        for arg in 1..args.len() {
            directories.push(args[arg].clone());
        }        
    }

    // Determine the paths to all files to process.
    let mut input_paths: Vec<String> = vec![];

    for directory in directories {
        let paths = fs::read_dir(directory).unwrap();
        for path in paths {
            match path {
                Ok(dir_entry) => {
                    match dir_entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                let pathname = dir_entry.path().as_os_str().to_str().unwrap().to_string();
                                if pathname.to_ascii_lowercase().ends_with(".png") {
                                    input_paths.push(pathname.clone());
                                }
                            }
                        },
                        Err(_err) => {}
                    }
                },
                Err(_err) => {}
            }
        }
    }

    // Read the contents of all files, and consolidate their palettes.
    let mut palette_map: HashMap<Rgb<u8>, u8> = HashMap::new();
    for pathname in &input_paths {
        let img = image::open(pathname.clone()).unwrap();
        let width = img.width();
        let height = img.height();
        println!("{}, {}x{}, {:?}", pathname, width, height, img.color());
    
        match img {
            image::DynamicImage::ImageRgba8(rgba) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x, y);
                        let a = pixel[3] >> 4;
                        if a > 0 {
                            let r = pixel[0] >> 4;
                            let g = pixel[1] >> 4;
                            let b = pixel[2] >> 4;
                            let color = Rgb::<u8>([r, g, b]);
                            if !palette_map.contains_key(&color) {
                                palette_map.insert(color, 0);
                            }    
                        }
                    }
                }
            },
            _ => {
                panic!("Unhandled image format. Must be RGBA8!");
            }
        }
    }

    println!("Palette needs {} custom colors.\n", palette_map.len() + 1);
    if palette_map.len() <= 240 {
        // Build a new indexed palette array.
        let mut palette_array: Vec<Rgb::<u8>> = vec![];

        // The color at index 0 is always considered transparent by VERA.
        // We throw in a useless black color at that palette index.
        let black = Rgb::<u8>([0, 0, 0]);
        palette_array.push(black.clone());

        // Also throw in a standard set of 15 colors.
        for _c in 1..16 {
            let white = Rgb::<u8>([15, 15, 15]);
            palette_array.push(white);
        }

        // Assign an index to every color in the palette.
        for color in palette_map.keys() {
            palette_array.push(*color);
        }

        // Remember the indexes so we can look them up by color.
        for index in 16..palette_array.len() {
            let color = palette_array[index];
            *palette_map.get_mut(&color).unwrap() = index as u8;
        }

        // Dump the palette to the console, for documentation purposes.
        println!("; Palette entries by index:");
        println!(";             VERA      Dec Hex:  R G B");
        println!(";");
        println!("begin_palette_table:");
        for index in 0..palette_array.len() {
            let color = palette_array[index];
            println!("    .byte    ${:x}{:x},$0{:x}  ; {:03} ${:02x}:  {:x} {:x} {:x}",
                color[1], color[2], color[0],
                index, index,
                color[0], color[1], color[2]);
        }
        for index in palette_array.len()..256 {
            let color = black.clone();
            println!("    .byte    ${:x}{:x},$0{:x}  ; {:03} ${:02x}:  {:x} {:x} {:x} (FREE)",
                color[1], color[2], color[0],
                index, index,
                color[0], color[1], color[2]);
        }
        println!("end_palette_table:\n");

        // For each PNG file, convert its pixels to palette indexes, and write to output file.
        for pathname in input_paths {
            let img = image::open(pathname.clone()).unwrap();
            let width = img.width();
            let height = img.height();
        
            match img {
                image::DynamicImage::ImageRgba8(rgba) => {
                    // Convert pixel colors into indexes.
                    let mut output_data: Vec<u8> = vec![];
                    for y in 0..height {
                        for x in 0..width {
                            let pixel = rgba.get_pixel(x, y);
                            let a = pixel[3] >> 4;
                            if a > 0 {
                                let r = pixel[0] >> 4;
                                let g = pixel[1] >> 4;
                                let b = pixel[2] >> 4;
                                let color = Rgb::<u8>([r, g, b]);
                                let index = palette_map.get(&color).unwrap();
                                output_data.push(*index);
                            } else {
                                output_data.push(0); // transparentfile:///home/curtis/images/64px/apple.bin

                            }
                        }
                    }

                    // Write the output data to a file.
                    let parts = pathname.split(".").collect::<Vec<&str>>();
                    let mut output_path = String::new();
                    for i in 0..parts.len()-1 {
                        output_path.push_str(parts[i]);
                        output_path.push_str(".");
                    }
                    output_path.push_str("bin");
                    match fs::File::create(output_path.clone()) {
                        Ok(mut file) => {
                            match file.write_all(&output_data[..]) {
                                Ok(()) => {
                                    println!("Wrote file ({}) as {} bytes.", output_path, output_data.len());
                                },
                                Err(err) => {
                                    println!("Cannot write output file ({}): {}", output_path, err.to_string());
                                }
                            }
                        },
                        Err(err) => {
                            println!("Cannot open output file ({}): {}", output_path, err.to_string());
                        }
                    }
                },
                _ => {
                    panic!("Unhandled image format. Must be RGBA8!");
                }
            }
        }
    } else {
        println!("Please reduce the number of colors used to 240 or less.");
    }
}
