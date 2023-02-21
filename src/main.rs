// Please see the README file for an overview of this program.
//
// Copyright 2023 by Curtis Whitley

use std::{fs, cmp::Ordering};
use std::io::Write;
use std::{env, vec};
use std::collections::HashMap;
use image::{Rgb};

const IMG_R: usize = 0;
const IMG_G: usize = 1;
const IMG_B: usize = 2;
const IMG_A: usize = 3;
const VRAM_PAGE_BOUNDARY: usize = 0x10000;
const VRAM_LIMIT: usize = 0x1F9C0;

#[derive(Debug, Clone)]
struct DirParameters {
    pub width: usize,
    pub height: usize,
    pub alignment: usize,
    pub bpp: u8,
    pub palette_offset: Option<usize>,
    pub no_output: bool,
    pub vapor: bool,
    pub path: String
}

impl DirParameters {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            alignment: 0,
            bpp: 0,
            palette_offset: None,
            no_output: false,
            vapor: false,
            path: String::new()        
        }
    }

    pub fn current_dir() -> Self {
        let mut params = DirParameters::new();
        params.alignment = 1;
        params.path = "./".to_string();
        params
    }
}

#[derive(Debug, Clone)]
struct FileParameters {
    pub width: usize,
    pub height: usize,
    pub alignment: usize,
    pub bpp: u8,
    pub palette_offset: Option<usize>,
    pub no_output: bool,
    pub vapor: bool,
    pub path: String,
    pub size: usize,
    pub max_colors: usize,
    pub colors: HashMap<Rgb<u8>, u8>,
    pub binary: Vec<u8>
}

impl FileParameters {
    pub fn new(params: &DirParameters) -> Self {
        Self {
            width: params.width,
            height: params.height,
            alignment: params.alignment,
            bpp: params.bpp,
            palette_offset: params.palette_offset,
            no_output: params.no_output,
            vapor: params.vapor,
            path: params.path.clone(),
            size: 0,
            max_colors: 0,
            colors: HashMap::new(),
            binary: vec![]
        }
    }
}

#[derive(Debug, Default)]
struct Expectations {
    pub width: bool,
    pub height: bool,
    pub file: bool,
    pub alignment: bool,
    pub bpp: bool,
    pub offset: bool
}

impl Expectations {
    pub fn new() -> Self {
        Expectations::default()
    }

    pub fn expect_file(&mut self) {
        *self = Expectations::new();
        self.file = true;
    }

    pub fn anything(&self) -> bool {
        self.width || self.height || self.alignment || self.bpp || self.offset
    }
}

fn main() {
    println!("Image to Binary (PNG-to-VERA file convertor) V1.5");

    // Determine which directories to use.
    let mut directories: Vec<DirParameters> = vec![];

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // No command arguments given; use current directory only.
        directories.push(DirParameters::current_dir());
    } else {
        // Traverse command arguments.
        let mut params = DirParameters::new();
        let mut expect = Expectations::new();
        expect.expect_file();

        for a in 1..args.len() {
            let arg = args[a].clone().to_ascii_lowercase();
            if arg.starts_with("-") && expect.anything() {
                println!("ERROR: Missing parameter value");
                return;
            } else if arg.eq("-w") | arg.eq("-width") {
                expect.width = true;
            } else if arg.eq("-h") || arg.eq("-height") {
                expect.height = true;
            } else if arg.eq("-a") || arg.eq("-alignment") {
                expect.alignment = true;
            } else if arg.eq("-b") || arg.eq("-bpp") {
                expect.bpp = true;
            } else if arg.eq("-p") || arg.eq("-paletteoffset") {
                expect.offset = true;
            } else if arg.eq("-n") || arg.eq("-nooutput") {
                params.no_output = true;
            } else if expect.width {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        params.width = number;
                        expect.expect_file();
                    },
                    Err(err) => {
                        println!("ERROR: Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else if expect.height {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        params.height = number;
                        expect.expect_file();
                    },
                    Err(err) => {
                        println!("ERROR: Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else if expect.alignment {
                match arg.as_str() {
                    "tb" => { params.alignment = 2048; },
                    "tilebase" => { params.alignment = 2048; },
                    "mb" => { params.alignment = 512; params.vapor = true; },
                    "mapbase" => { params.alignment = 512; params.vapor = true; },
                    "sp" => { params.alignment = 32; },
                    "sprite" => { params.alignment = 32; },
                    "bm" => { params.alignment = 2048; },
                    "bitmap" => { params.alignment = 2048; },
                    _ => {
                        match arg.parse::<usize>() {
                            Ok(number) => {
                                params.alignment = number;
                            },
                            Err(err) => {
                                println!("ERROR: Invalid alignment: {}", err.to_string());
                                return;
                            }
                        }        
                    }
                }
                expect.expect_file();
            } else if expect.bpp {
                match arg.parse::<u8>() {
                    Ok(number) => {
                        if number == 1 || number == 2 || number == 4 || number == 8 {
                            params.bpp = number;
                            expect.expect_file();    
                        } else {
                            println!("ERROR: Invalid bits-per-pixel");
                            return;
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Invalid bits-per-pixel: {}", err.to_string());
                        return;
                    }
                }
            } else if expect.offset {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        if number >= 1 && number <= 15 {
                            params.palette_offset = Some(number);
                            expect.expect_file();    
                        } else {
                            println!("ERROR: Invalid palette offset");
                            return;
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Invalid palette offset: {}", err.to_string());
                        return;
                    }
                }
            } else {
                params.path = arg;
                directories.push(params);
                params = DirParameters::new();
                expect = Expectations::new();
            }
        }        

        if expect.anything() {
            println!("ERROR: Missing parameter value");
            return;
        } else if expect.file {
            params.path = "./".to_string();
            directories.push(params);
        }
    }

    // Make sure we have something to do.
    if directories.len() == 0 {
        println!("ERROR: No directories to process.");
        return;
    }

    // Determine the paths to all files to process.
    let mut files: Vec<FileParameters> = vec![];

    for directory in &mut directories {
        // Validate certain options.
        if directory.bpp == 0 {
            directory.bpp = 8;
        }
        if directory.palette_offset.is_some() && directory.bpp == 8 {
            println!("ERROR: Do not specify palette offset with 8 bits-per-pixel");
            return;
        }
        if directory.palette_offset.is_none() && directory.bpp != 8 {
            println!("ERROR: Please specify palette offset with 1/2/4 bits-per-pixel");
            return;
        }

        // Skip virtual data, as there is no directory or file.
        if directory.vapor {
            let mut params = FileParameters::new(&directory);
            params.size = directory.width * directory.height * 2;
            files.push(params);
            continue;
        }

        println!("Reading: {}", directory.path);

        // Check for accessing a single file, rather than a directory.
        if directory.path.to_ascii_lowercase().ends_with(".png") {
            match fs::metadata(directory.path.clone()) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let img = image::open(directory.path.clone()).unwrap();
                        let mut params = FileParameters::new(&directory);
                        if directory.width == 0 {
                            params.width = img.width() as usize;
                        }
                        if directory.height == 0 {
                            params.height = img.height() as usize;
                        }

                        let mut width = params.width;
                        match params.bpp {
                            1 => {
                                width = (width + 7) / 8;
                            },
                            2 => {
                                width = (width + 3) / 4;
                            },
                            4 => {
                                width = (width + 1) / 2;
                            },
                            _ => {}
                        }
                        params.size = width * params.height;

                        files.push(params);
                    } else {
                        println!("ERROR: Specified file is not a file: {}", directory.path);
                        return;
                    }
                },
                Err(_) => {
                    println!("ERROR: Cannot read the specified file: {}", directory.path);
                    return;
                }
            }
            continue;
        }

        // We must be accessing a whole directory.
        let paths = match fs::read_dir(&directory.path) {
            Ok(path) => path,
            Err(_) => {
                println!("ERROR: Cannot read the specified directory: {}", directory.path);
                return;
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
                                    let mut params = FileParameters::new(&directory);
                                    if directory.width == 0 {
                                        params.width = img.width() as usize;
                                    }
                                    if directory.height == 0 {
                                        params.height = img.height() as usize;
                                    }

                                    let mut width = params.width;
                                    match params.bpp {
                                        1 => {
                                            width = (width + 7) / 8;
                                        },
                                        2 => {
                                            width = (width + 3) / 4;
                                        },
                                        4 => {
                                            width = (width + 1) / 2;
                                        },
                                                    _ => {}
                                    }
                                    params.size = width * params.height;
                                                                        
                                    files.push(params);
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
        println!("ERROR: No files to process.");
        return;
    }

    // Read the contents of all files, and determine their unique pixel colors.
    for img_file in &mut files {
        // Determine the maximum number of colors, not including transparent
        match img_file.bpp {
            0 => {
                img_file.bpp = 8;
                img_file.max_colors = 239;
            },
            1 => {
                img_file.max_colors = 1;
            },
            2 => {
                img_file.max_colors = 3;
            },
            4 => {
                img_file.max_colors = 15;
            },
            8 => {
                img_file.max_colors = 239;
            },
            _ => {}
        }

        // Use default alignment, if needed
        if img_file.alignment == 0 {
            img_file.alignment = 1;
        }

        // Check for needing to read the file
        if img_file.vapor {
            continue; // skip it
        }

        // Read the file contents
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
                        if !img_file.colors.contains_key(&color) {
                            if img_file.colors.len() >= img_file.max_colors {
                                println!("ERROR: File {} contains too many colors (over {})",
                                    img_file.path, img_file.max_colors);
                                return;
                            }
                            let index = (img_file.colors.len() + 1) as u8;
                            img_file.colors.insert(color, index);
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
                            if !img_file.colors.contains_key(&color) {
                                if img_file.colors.len() >= img_file.max_colors {
                                    println!("ERROR: File {} contains too many colors (over {})",
                                        img_file.path, img_file.max_colors);
                                    return;
                                }
                                let index = (img_file.colors.len() + 1) as u8;
                                img_file.colors.insert(color, index);
                            }
                        }
                    }
                }
            },
            _ => {
                println!("ERROR: Unhandled image format ({}). Must be RGB8 or RGBA8!", img_file.path);
                return;
            }
        }

        println!("File {} has {} unique colors (maximum is {}).",
            img_file.path, img_file.colors.len(), img_file.max_colors);
    }

    // Use the colors of all files, and consolidate their palettes.

    let mut palette_map: HashMap<Rgb<u8>, Vec<u8>> = HashMap::new();
    let mut offset_map: Vec<HashMap<Rgb<u8>, u8>> = vec![];
    let mut palette_array: Vec<Option<Rgb::<u8>>> = vec![];

    for _offset in 0..16 {
        offset_map.push(HashMap::new());
    }

    for _index in 0..256 {
        palette_array.push(None);
    }

    // Insert the standard palette colors.
    palette_map.insert(Rgb::<u8>([15, 15, 15]), [1].to_vec());
    palette_map.insert(Rgb::<u8>([8, 0, 0]), [2].to_vec());
    palette_map.insert(Rgb::<u8>([10, 15, 14]), [3].to_vec());
    palette_map.insert(Rgb::<u8>([12, 4, 12]), [4].to_vec());
    palette_map.insert(Rgb::<u8>([0, 12, 5]), [5].to_vec());
    palette_map.insert(Rgb::<u8>([0, 0, 10]), [6].to_vec());
    palette_map.insert(Rgb::<u8>([14, 14, 7]), [7].to_vec());
    palette_map.insert(Rgb::<u8>([13, 8, 5]), [8].to_vec());
    palette_map.insert(Rgb::<u8>([6, 4, 0]), [9].to_vec());
    palette_map.insert(Rgb::<u8>([15, 7, 7]), [10].to_vec());
    palette_map.insert(Rgb::<u8>([3, 3, 3]), [11].to_vec());
    palette_map.insert(Rgb::<u8>([7, 7, 7]), [12].to_vec());
    palette_map.insert(Rgb::<u8>([10, 15, 6]), [13].to_vec());
    palette_map.insert(Rgb::<u8>([0, 8, 15]), [14].to_vec());
    palette_map.insert(Rgb::<u8>([11, 11, 11]), [15].to_vec());

    // Consolidate shared palette offset colors.
    for img_file in &files {
        match img_file.palette_offset {
            Some(offset) => {
                for (color, index) in &img_file.colors {
                    if !offset_map[offset].contains_key(&color) {
                        offset_map[offset].insert(color.clone(), *index);
                    }
                }
            },
            None => {}
        }
    }

    // Place colors from palette offsets into the overall palette map.
    for offset in 1..16 {
        for (color, index) in &offset_map[offset] {
            let palette_index = (offset * 16) as u8 + index;
            match palette_map.get_mut(&color) {
                Some(indexes) => {
                    if !indexes.contains(&palette_index) {
                        indexes.push(palette_index);
                    }
                },
                None => {
                    let mut indexes: Vec<u8> = vec![];
                    indexes.push(palette_index);
                    palette_map.insert(color.clone(), indexes);
                }
            }
        }
    }

    // Copy standard and offset colors to the palette array.
    for (color, indexes) in &palette_map {
        for index in indexes {
            palette_array[*index as usize] = Some(color.clone());
        }
    }

    // Find indexes for all non-palette-offset (i.e., 8-bpp) colors.
    let next_index: usize = 16;
    for img_file in &mut files {
        match img_file.palette_offset {
            None => {
                for color in img_file.colors.keys() {
                    if !palette_map.contains_key(color) {
                        let mut found = false;
                        for palette_index in next_index..256 {
                            if palette_array[palette_index].is_none() {
                                palette_array[palette_index] = Some(color.clone());
                                let mut indexes: Vec<u8> = vec![];
                                indexes.push(palette_index as u8);
                                palette_map.insert(color.clone(), indexes);            
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            println!("ERROR: Could not insert all colors into palette (please reduce colors)");
                            return;
                        }
                    }        
                }
            },
            _ => {}
        }
    }

    // Dump the palette to the console, for documentation purposes.
    println!("; Palette entries by index:");
    println!(";             VERA      Dec Hex:  R G B");
    println!(";");
    println!("begin_palette_table:");
    for index in 0..palette_array.len() {
        let color: Rgb<u8>;        
        let free = match palette_array[index] {
            Some(c) => {
                color = c.clone();
                ""
            },
            None => {
                color = Rgb::<u8>([0,0,0]); // black
                " (FREE)"
            }
        };
        println!("    .byte    ${:x}{:x},$0{:x}  ; {:03} ${:02x}:  {:x} {:x} {:x}{}",
            color[1], color[2], color[0], // G B R
            index, index,
            color[0], color[1], color[2], // R G B
            free);
    }
    println!("end_palette_table:\n");

    // For each PNG file, convert its pixels to palette indexes, and write to output file.
    for img_file in &mut files {
        if img_file.vapor || img_file.no_output {
            continue; // skip it
        }
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
            image::DynamicImage::ImageRgb8(rgb) => {
                // Convert pixel colors into indexes.
                let mut output_data: Vec<u8> = vec![];
                output_data.push(0); // dummy address LO
                output_data.push(0); // dummy address HI

                let mask: u8 = match img_file.bpp {
                    1 => 1,
                    2 => 3,
                    4 => 15,
                    8 => 255,
                    _ => 0
                };

                for out_y in out_start_y..out_end_y {
                    let mut bits_used: u8 = 0;
                    let mut output_byte: u8 = 0;
        
                    let img_y = img_center_y - (out_center_y - out_y);
                    if img_y < 0 || img_y >= img_height {
                        for _out_x in out_start_x..out_end_x {
                            // output transparent color index (zero)
                            if img_file.bpp == 8 {
                                output_data.push(0);
                                img_file.binary.push(0);
                            } else {
                                output_byte <<= img_file.bpp;
                                bits_used += img_file.bpp;
                                if bits_used >= 8 {
                                    output_data.push(output_byte);
                                    img_file.binary.push(output_byte);
                                    output_byte = 0;
                                    bits_used = 0;
                                }    
                            }
                        }
                    } else {
                        for out_x in out_start_x..out_end_x {
                            let img_x = img_center_x - (out_center_x - out_x);
                            if img_x < 0 || img_x >= img_width {
                                // output transparent color index (zero)
                                if img_file.bpp == 8 {
                                    output_data.push(0);
                                    img_file.binary.push(0);
                                } else {
                                    output_byte <<= img_file.bpp;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 {
                                        output_data.push(output_byte);
                                        img_file.binary.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            } else {
                                let pixel = rgb.get_pixel(img_x as u32, img_y as u32);
                                let r = pixel[IMG_R] >> 4;
                                let g = pixel[IMG_G] >> 4;
                                let b = pixel[IMG_B] >> 4;
                                let color = Rgb::<u8>([r, g, b]);

                                let index = match img_file.palette_offset {
                                    Some(offset) => {
                                        // 1/2/4 bpp
                                        offset_map[offset].get(&color).unwrap().clone() & mask
                                    }
                                    None => {
                                        // 8bpp
                                        let indexes = palette_map.get(&color).unwrap();
                                        indexes[0]
                                    }
                                };

                                // output some color index
                                if img_file.bpp == 8 {
                                    output_data.push(index);
                                    img_file.binary.push(index);
                                } else {
                                    output_byte = (output_byte << img_file.bpp) | index;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 {
                                        output_data.push(output_byte);
                                        img_file.binary.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            }
                        }    
                    }
                    // finish the pixel row
                    if bits_used > 0 {
                        while bits_used < 8 {
                            output_byte <<= img_file.bpp;
                            bits_used += img_file.bpp;
                        }
                        output_data.push(output_byte);
                        img_file.binary.push(output_byte);
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
                                println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            image::DynamicImage::ImageRgba8(rgba) => {
                // Convert pixel colors into indexes.
                let mut output_data: Vec<u8> = vec![];
                output_data.push(0); // dummy address LO
                output_data.push(0); // dummy address HI

                let mask: u8 = match img_file.bpp {
                    1 => 1,
                    2 => 3,
                    4 => 15,
                    8 => 255,
                    _ => 0
                };

                for out_y in out_start_y..out_end_y {
                    let mut bits_used: u8 = 0;
                    let mut output_byte: u8 = 0;
        
                    let img_y = img_center_y - (out_center_y - out_y);
                    if img_y < 0 || img_y >= img_height {
                        for _out_x in out_start_x..out_end_x {
                            // output transparent color index (zero)
                            if img_file.bpp == 8 {
                                output_data.push(0);
                                img_file.binary.push(0);
                            } else {
                                output_byte <<= img_file.bpp;
                                bits_used += img_file.bpp;
                                if bits_used >= 8 {
                                    output_data.push(output_byte);
                                    img_file.binary.push(output_byte);
                                    output_byte = 0;
                                    bits_used = 0;
                                }    
                            }
                        }
                    } else {
                        for out_x in out_start_x..out_end_x {
                            let img_x = img_center_x - (out_center_x - out_x);
                            if img_x < 0 || img_x >= img_width {
                                // output transparent color index (zero)
                                if img_file.bpp == 8 {
                                    output_data.push(0);
                                    img_file.binary.push(0);
                                } else {
                                    output_byte <<= img_file.bpp;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 {
                                        output_data.push(output_byte);
                                        img_file.binary.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            } else {
                                let pixel = rgba.get_pixel(img_x as u32, img_y as u32);
                                let a = pixel[IMG_A] >> 4;
                                if a > 0 {
                                    let r = pixel[IMG_R] >> 4;
                                    let g = pixel[IMG_G] >> 4;
                                    let b = pixel[IMG_B] >> 4;
                                    let color = Rgb::<u8>([r, g, b]);
    
                                    let index = match img_file.palette_offset {
                                        Some(offset) => {
                                            // 1/2/4 bpp
                                            offset_map[offset].get(&color).unwrap().clone() & mask
                                        }
                                        None => {
                                            // 8bpp
                                            let indexes = palette_map.get(&color).unwrap();
                                            indexes[0]
                                        }
                                    };
    
                                    // output some color index
                                    if img_file.bpp == 8 {
                                        output_data.push(index);
                                        img_file.binary.push(index);
                                    } else {
                                        output_byte = (output_byte << img_file.bpp) | index;
                                        bits_used += img_file.bpp;
                                        if bits_used >= 8 {
                                            output_data.push(output_byte);
                                            img_file.binary.push(output_byte);
                                            output_byte = 0;
                                            bits_used = 0;
                                        }    
                                    }
                                } else {
                                    // output transparent color index (zero)
                                    if img_file.bpp == 8 {
                                        output_data.push(0);
                                        img_file.binary.push(0);
                                    } else {
                                        output_byte <<= img_file.bpp;
                                        bits_used += img_file.bpp;
                                        if bits_used >= 8 {
                                            output_data.push(output_byte);
                                            img_file.binary.push(output_byte);
                                            output_byte = 0;
                                            bits_used = 0;
                                        }    
                                    }
                                }
                            }
                        }    
                    }
                    // finish the pixel row
                    if bits_used > 0 {
                        while bits_used < 8 {
                            output_byte <<= img_file.bpp;
                            bits_used += img_file.bpp;
                        }
                        output_data.push(output_byte);
                        img_file.binary.push(output_byte);
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
                                println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            _ => {
                panic!("ERROR: Unhandled image format. Must be RGBA8!");
            }
        }
    }

    // Write the palette data to a file.
    let mut palette_bytes: Vec<u8> = vec![];
    // 2-byte address offset
    palette_bytes.push(0); // dummy address LO
    palette_bytes.push(0); // dummy address HI
    // standard and custom colors
    for index in 0..palette_array.len() {
        match palette_array[index] {
            Some(color) => {
                // Output: [ggggbbbb] [----rrrr]
                palette_bytes.push((color[1]<<4)|color[2]); // G B
                palette_bytes.push(color[0]); // R
            },
            None => {
                palette_bytes.push(0);
                palette_bytes.push(0);
            }
        }
    }

    let uc_path = "PALETTE.BIN".to_string();
    match fs::File::create(uc_path.clone()) {
        Ok(mut file) => {
            match file.write_all(&palette_bytes[..]) {
                Ok(()) => {
                    println!("Wrote file ({}) as {} bytes.", uc_path, palette_bytes.len());
                },
                Err(err) => {
                    println!("ERROR: Cannot write palette file ({}): {}", uc_path, err.to_string());
                }
            }
        },
        Err(err) => {
            println!("ERROR: Cannot open palette file ({}): {}", uc_path, err.to_string());
        }
    }

    arrange_files_in_memory(&mut files);
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

fn section_filename(path: &str, section: &str) -> String {
    let parts = path.split("/").collect::<Vec<&str>>();
    let mut output_path = String::new();
    for i in 0..parts.len()-1 {
        output_path.push_str(parts[i]);
        output_path.push_str("/");
    }

    let parts2 = parts[parts.len()-1].split(".").collect::<Vec<&str>>();
    for i in 0..parts2.len()-1 {
        if i > 0 {
            output_path.push_str(".");
        }
        output_path.push_str(&parts2[i].to_ascii_uppercase());
    }
    output_path.push_str("P");
    output_path.push_str(section);
    output_path.push_str(".BIN");

    output_path
}

fn arrange_files_in_memory(files: &mut Vec<FileParameters>) {
    // Sort the files based on:
    // - vapor flag (descending)
    // - alignment (descending)
    // - size (descending)
    // - path (ascending)
    files.sort_by(|a,b| {
        if a.vapor > b.vapor {
            Ordering::Less
        } else if a.vapor < b.vapor {
            Ordering::Greater
        } else if a.alignment > b.alignment {
            Ordering::Less
        } else if a.alignment < b.alignment {
            Ordering::Greater
        } else if a.size > b.size {
            Ordering::Less
        } else if a.size < b.size {
            Ordering::Greater
        } else {
            a.path.partial_cmp(&b.path).unwrap()
        }
    });

    // Try to fit the series of files into VRAM based on their
    // specified (or assumed) alignment values.

    println!("\nVRAM Address Arrangement\n");
    println!("Waste Start  End    Size  Align Width Height Path/Name");
    println!("----- ------ ------ ----- ----- ----- ------ ----------------------------------");

    let mut boundary_crossing = false;
    let mut address: usize = 0;
    loop {
        if files.len() == 0 {
            break; // no more files to arrange
        }
        let file = files[0].clone();

        // Advance the address, if needed, based on alignment.
        let next_address = ((address + file.alignment - 1) / file.alignment) * file.alignment;
        let diff = next_address - address;

        if diff == 0 || files.len() == 1 {
            // The current file fits perfectly at the next address,
            // or this is the last file to arrange.
            let last_address = next_address + file.size - 1;
            println!("{:5} ${:05x} ${:05x} {:5} {:5} {:5} {:5}  {}",
                diff,
                next_address,
                last_address,
                file.size,
                file.alignment,
                file.width,
                file.height,
                file.path);

            boundary_crossing |=
                check_for_vram_page_crossing(next_address,
                    last_address, &file);

            files.remove(0);
            address = next_address + file.size;
        } else {
            // Find the file whose size uses the difference the best.
            let mut waste_diff = diff;
            let mut best_index: usize = 0;
            let mut best_diff = diff;
            let mut best_address: usize = next_address;

            for i in 1..files.len() {
                let file2 = files[i].clone();

                // Align this potential next file
                let next_address2 =
                    ((address + file2.alignment - 1) / file2.alignment) * file2.alignment;
                let diff2 = next_address2 - address;

                // Realign the file in question
                let next_address3 = next_address2 + file2.size;
                let next_address4 =
                    ((next_address3 + file.alignment - 1) /
                        file.alignment) * file.alignment;
                let diff4 = next_address4 - next_address3 + diff2;

                if diff4 < best_diff {
                    waste_diff = diff2;
                    best_index = i;
                    best_diff = diff4;
                    best_address = next_address2;
                }
            }

            // Reorder the files by using the best fit file next
            let file2 = files[best_index].clone();
            files.remove(best_index);

            // The current file fits perfectly at the next address,
            // or this is the last file to arrange.
            let last_address = best_address + file2.size - 1;
            println!("{:5} ${:05x} ${:05x} {:5} {:5} {:5} {:5}  {}",
                waste_diff,
                best_address,
                last_address,
                file2.size,
                file2.alignment,
                file2.width,
                file2.height,
                file2.path);

            boundary_crossing |=
                check_for_vram_page_crossing(best_address,
                    last_address, &file);

            address = best_address + file2.size;
        }
    }
    if boundary_crossing {
        println!("");
        println!("NOTE: one output image crosses the VRAM page boundary, so there are now two");
        println!("      extra output files, for loading the data in two sections, if needed.");
    }
    if address > VRAM_LIMIT {
        println!("");
        println!("ERROR: These files will not fit in VRAM together.");
    }
}

fn check_for_vram_page_crossing(first_address: usize, last_address: usize,
                                img_file: &FileParameters) -> bool {
    if first_address < VRAM_PAGE_BOUNDARY && last_address > VRAM_PAGE_BOUNDARY {
        // Output file data crosses VRAM page boundary.
        // We need to output 2 extra files to split the data for loading.
        let bank_0_size = VRAM_PAGE_BOUNDARY - first_address;
        let bank_1_size = last_address + 1 - VRAM_PAGE_BOUNDARY;
        let bank_0_first_address = first_address;
        let bank_0_last_address = VRAM_PAGE_BOUNDARY - 1;
        let bank_1_first_address = VRAM_PAGE_BOUNDARY;
        let bank_1_last_address = VRAM_PAGE_BOUNDARY + bank_1_size - 1;

        // Write the 1st part of the data.
        let mut section_data = img_file.binary[..bank_0_size].to_vec();
        let mut output_data: Vec<u8> = vec![];
        output_data.push(0); // dummy address byte
        output_data.push(0); // dummy address byte
        output_data.append(&mut section_data);
        let uc_path = section_filename(&img_file.path, "0");
        match fs::File::create(uc_path.clone()) {
            Ok(mut file) => {
                match file.write_all(&output_data[..]) {
                    Ok(()) => {
                        println!("      ${:05x} ${:05x} {:5}                    {}",
                            bank_0_first_address,
                            bank_0_last_address,
                            bank_0_size,
                            uc_path);
                    },
                    Err(err) => {
                        println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            Err(err) => {
                println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
            }
        }
        
        // Write the 2nd part of the data.
        let mut section_data = img_file.binary[bank_0_size..].to_vec();
        let mut output_data: Vec<u8> = vec![];
        output_data.push(0); // dummy address byte
        output_data.push(0); // dummy address byte
        output_data.append(&mut section_data);
        let uc_path = section_filename(&img_file.path, "1");
        match fs::File::create(uc_path.clone()) {
            Ok(mut file) => {
                match file.write_all(&output_data[..]) {
                    Ok(()) => {
                        println!("      ${:05x} ${:05x} {:5}                    {}",
                            bank_1_first_address,
                            bank_1_last_address,
                            bank_1_size,
                            uc_path);
                    },
                    Err(err) => {
                        println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            Err(err) => {
                println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
            }
        }
        true
    } else {
        false
    }
}