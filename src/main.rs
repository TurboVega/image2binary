// Please see the README file for an overview of this program.
//
// Copyright 2023 by Curtis Whitley

use std::fs;
use std::io::Write;
use std::env;
use std::collections::HashMap;
use image::{Rgb};

const IMG_R: usize = 0;
const IMG_G: usize = 1;
const IMG_B: usize = 2;
const IMG_A: usize = 3;

struct Parameters {
    pub width: usize,
    pub height: usize,
    pub path: String
}

fn main() {
    println!("Image to Binary (PNG file convertor) V1.1");

    // Determine which directories to use.
    let mut directories: Vec<Parameters> = vec![];

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // No command arguments given; use current directory only.
        directories.push(Parameters { height: 0, width: 0, path: "./".to_string() });
    } else {
        // Traverse command arguments.
        let mut width = 0;
        let mut height = 0;
        let mut expect_width = false;
        let mut expect_height = false;
        let mut expect_file = false;
        for a in 1..args.len() {
            let arg = args[a].clone();
            if arg.to_ascii_lowercase().eq("-w") {
                if expect_width || expect_height {
                    println!("Missing width/height value");
                    return;
                }
                expect_width = true;
            } else if arg.to_ascii_lowercase().eq("-h") {
                if expect_width || expect_height {
                    println!("Missing width/height value");
                    return;
                }
                expect_height = true;
            } else if expect_width {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        width = number;
                        expect_width = false;
                        expect_file = true;
                    },
                    Err(err) => {
                        println!("Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else if expect_height {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        height = number;
                        expect_height = false;
                        expect_file = true;
                    },
                    Err(err) => {
                        println!("Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else {
                directories.push(Parameters { width, height, path: arg });
                width = 0;
                height = 0;
                expect_file = false;
            }
        }        

        if expect_file {
            directories.push(Parameters { width, height, path: "./".to_string() });
        } else if expect_width || expect_height {
            println!("Missing width/height parameter");
            return;
        }
    }

    // Make sure we have something to do.
    if directories.len() == 0 {
        println!("No directories to process.");
        return;
    }

    // Determine the paths to all files to process.
    let mut files: Vec<Parameters> = vec![];

    for directory in &directories {
        println!("Reading: {}", directory.path);
        let paths = match fs::read_dir(&directory.path) {
            Ok(path) => path,
            Err(_) => {
                println!("Cannot read the specified directory.");
                continue;
            }
        };
        for path in paths {
            match path {
                Ok(dir_entry) => {
                    match dir_entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                let pathname = dir_entry.path().as_os_str().to_str().unwrap().to_string();
                                if pathname.to_ascii_lowercase().ends_with(".png") {
                                    let img = image::open(pathname.clone()).unwrap();
                                    files.push(Parameters {
                                        width: match directory.width {
                                            0 => img.width() as usize,
                                            _ => directory.width
                                        },
                                        height: match directory.height {
                                            0 => img.height() as usize,
                                            _ => directory.height
                                        },
                                        path: pathname.clone()
                                    });
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

    // Make sure we have something to do.
    if files.len() == 0 {
        println!("No files to process.");
        return;
    }

    // Read the contents of all files, and consolidate their palettes.
    let mut palette_map: HashMap<Rgb<u8>, u8> = HashMap::new();
    for img_file in &files {
        let img = image::open(img_file.path.clone()).unwrap();
        let width = img.width();
        let height = img.height();
        println!("{}, {}x{}, {:?}", img_file.path, width, height, img.color());
    
        match img {
            image::DynamicImage::ImageRgb8(rgba) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x, y);
                        let r = pixel[IMG_R] >> 4;
                        let g = pixel[IMG_G] >> 4;
                        let b = pixel[IMG_B] >> 4;
                        let color = Rgb::<u8>([r, g, b]);
                        if !palette_map.contains_key(&color) {
                            palette_map.insert(color, 0);
                        }    
                    }
                }
            },
            image::DynamicImage::ImageRgba8(rgba) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x, y);
                        let a = pixel[IMG_A] >> 4;
                        if a > 0 {
                            let r = pixel[IMG_R] >> 4;
                            let g = pixel[IMG_G] >> 4;
                            let b = pixel[IMG_B] >> 4;
                            let color = Rgb::<u8>([r, g, b]);
                            if !palette_map.contains_key(&color) {
                                palette_map.insert(color, 0);
                            }    
                        }
                    }
                }
            },
            _ => {
                panic!("Unhandled image format ({:?}). Must be RGB8 or RGBA8!", img);
            }
        }
    }

    println!("Palette needs {} custom colors.\n", palette_map.len() + 1);
    if palette_map.len() <= 240 {
        // Build a new indexed palette array.
        let mut palette_array: Vec<Rgb::<u8>> = vec![];

        // The color at index 0 is always considered transparent by VERA.
        // We throw in a useless black color at that palette index.
        let black = Rgb::<u8>([0, 0, 0]);      // 0
        palette_array.push(black.clone());

        // Also throw in a standard set of 15 colors (RGB).
        palette_array.push(Rgb::<u8>([15, 15, 15]));    // 1
        palette_array.push(Rgb::<u8>([8, 0, 0]));       // 2
        palette_array.push(Rgb::<u8>([10, 15, 14]));    // 3
        palette_array.push(Rgb::<u8>([12, 4, 12]));     // 4
        palette_array.push(Rgb::<u8>([0, 12, 5]));      // 5
        palette_array.push(Rgb::<u8>([0, 0, 10]));      // 6
        palette_array.push(Rgb::<u8>([14, 14, 7]));     // 7
        palette_array.push(Rgb::<u8>([13, 8, 5]));      // 8
        palette_array.push(Rgb::<u8>([6, 4, 0]));       // 9
        palette_array.push(Rgb::<u8>([15, 7, 7]));      // 10
        palette_array.push(Rgb::<u8>([3, 3, 3]));       // 11
        palette_array.push(Rgb::<u8>([7, 7, 7]));       // 12
        palette_array.push(Rgb::<u8>([10, 15, 6]));     // 13
        palette_array.push(Rgb::<u8>([0, 8, 15]));      // 14
        palette_array.push(Rgb::<u8>([11, 11, 11]));    // 15

        // Assign an index to every custom color in the palette.
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
                color[1], color[2], color[0], // G B R
                index, index,
                color[0], color[1], color[2]); // R G B
        }
        for index in palette_array.len()..256 {
            let color = black.clone();
            println!("    .byte    ${:x}{:x},$0{:x}  ; {:03} ${:02x}:  {:x} {:x} {:x} (FREE)",
                color[1], color[2], color[0], // G B R
                index, index,
                color[0], color[1], color[2]); // R G B
        }
        println!("end_palette_table:\n");

        // For each PNG file, convert its pixels to palette indexes, and write to output file.
        for img_file in files {
            let img = image::open(img_file.path.clone()).unwrap();

            // Get dimensions for input image.
            let img_width = img.width() as i32;
            let img_height = img.height() as i32;
            let img_center_x = img_width / 2;
            let img_center_y = img_height / 2;

            // Get dimensions for output image.
            let out_width = img_file.width as i32;
            let out_height = img_file.height as i32;
            let out_center_x = out_width / 2;
            let out_center_y = out_height / 2;

            // Compute necessary rectangles.
            let out_start_x: i32;
            let out_end_x: i32;
            let out_start_y: i32;
            let out_end_y: i32;

            out_start_x = 0;
            out_end_x = out_width;
        
            out_start_y = 0;
            out_end_y = out_height;
        
            match img {
                image::DynamicImage::ImageRgb8(rgba) => {
                    // Convert pixel colors into indexes.
                    let mut output_data: Vec<u8> = vec![];
                    for out_y in out_start_y..out_end_y {
                        let img_y = img_center_y - (out_center_y - out_y);
                        if img_y < 0 || img_y >= img_height {
                            for _out_x in out_start_x..out_end_x {
                                output_data.push(0); // transparent
                            }
                        } else {
                            for out_x in out_start_x..out_end_x {
                                let img_x = img_center_x - (out_center_x - out_x);
                                if img_x < 0 || img_x >= img_width {
                                    output_data.push(0); // transparent
                                } else {
                                    let pixel = rgba.get_pixel(img_x as u32, img_y as u32);
                                    let r = pixel[IMG_R] >> 4;
                                    let g = pixel[IMG_G] >> 4;
                                    let b = pixel[IMG_B] >> 4;
                                    let color = Rgb::<u8>([r, g, b]);
                                    let index = palette_map.get(&color).unwrap();
                                    output_data.push(*index);
                                }
                            }    
                        }
                    }

                    // Write the output data to a file.
                    let uc_path = upcase_filename(&img_file.path);
                    match fs::File::create(uc_path.clone()) {
                        Ok(mut file) => {
                            match file.write_all(&output_data[..]) {
                                Ok(()) => {
                                    println!("Wrote file ({}) as {} bytes.", uc_path, output_data.len());
                                },
                                Err(err) => {
                                    println!("Cannot write output file ({}): {}", uc_path, err.to_string());
                                }
                            }
                        },
                        Err(err) => {
                            println!("Cannot open output file ({}): {}", uc_path, err.to_string());
                        }
                    }
                },
                image::DynamicImage::ImageRgba8(rgba) => {
                    // Convert pixel colors into indexes.
                    let mut output_data: Vec<u8> = vec![];
                    for out_y in out_start_y..out_end_y {
                        let img_y = img_center_y - (out_center_y - out_y);
                        if img_y < 0 || img_y >= img_height {
                            for _out_x in out_start_x..out_end_x {
                                output_data.push(0); // transparent
                            }
                        } else {
                            for out_x in out_start_x..out_end_x {
                                let img_x = img_center_x - (out_center_x - out_x);
                                if img_x < 0 || img_x >= img_width {
                                    output_data.push(0); // transparent
                                } else {
                                    let pixel = rgba.get_pixel(img_x as u32, img_y as u32);
                                    let a = pixel[IMG_A] >> 4;
                                    if a > 0 {
                                        let r = pixel[IMG_R] >> 4;
                                        let g = pixel[IMG_G] >> 4;
                                        let b = pixel[IMG_B] >> 4;
                                        let color = Rgb::<u8>([r, g, b]);
                                        let index = palette_map.get(&color).unwrap();
                                        output_data.push(*index);
                                    } else {
                                        output_data.push(0); // transparent
                                    }    
                                }
                            }    
                        }
                    }

                    // Write the output data to a file.
                    let uc_path = upcase_filename(&img_file.path);
                    match fs::File::create(uc_path.clone()) {
                        Ok(mut file) => {
                            match file.write_all(&output_data[..]) {
                                Ok(()) => {
                                    println!("Wrote file ({}) as {} bytes.", uc_path, output_data.len());
                                },
                                Err(err) => {
                                    println!("Cannot write output file ({}): {}", uc_path, err.to_string());
                                }
                            }
                        },
                        Err(err) => {
                            println!("Cannot open output file ({}): {}", uc_path, err.to_string());
                        }
                    }
                },
                _ => {
                    panic!("Unhandled image format. Must be RGBA8!");
                }
            }
        }

        // Write the palette data to a file.
        let mut palette_bytes: Vec<u8> = vec![];
        // 2-byte address offset
        palette_bytes.push(0);
        palette_bytes.push(0);
        // standard and custom colors
        for index in 0..palette_array.len() {
            let color = palette_array[index];
            // Output: [ggggbbbb] [----rrrr]
            palette_bytes.push((color[1]<<4)|color[2]); // G B
            palette_bytes.push(color[0]); // R
        }
        // unused (free) colors
        for _index in palette_array.len()..256 {
            palette_bytes.push(0);
            palette_bytes.push(0);
        }

        let uc_path = "PALETTE.BIN".to_string();
        match fs::File::create(uc_path.clone()) {
            Ok(mut file) => {
                match file.write_all(&palette_bytes[..]) {
                    Ok(()) => {
                        println!("Wrote file ({}) as {} bytes.", uc_path, palette_bytes.len());
                    },
                    Err(err) => {
                        println!("Cannot write palette file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            Err(err) => {
                println!("Cannot open palette file ({}): {}", uc_path, err.to_string());
            }
        }
    } else {
        println!("Please reduce the number of colors used to 240 or less.");
    }
}

fn upcase_filename(path: &str) -> String {
    let parts = path.split("/").collect::<Vec<&str>>();
    let mut output_path = String::new();
    for i in 0..parts.len()-1 {
        output_path.push_str(parts[i]);
        output_path.push_str("/");
    }

    let parts2 = parts[parts.len()-1].split(".").collect::<Vec<&str>>();
    for i in 0..parts2.len()-1 {
        output_path.push_str(&parts2[i].to_ascii_uppercase());
        output_path.push_str(".");
    }
    output_path.push_str("BIN");

    output_path
}
