#[macro_use]
extern crate clap;
extern crate image;

use std::fs::File;

use clap::App;
use image::ImageFormat;

fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();

    let input = arguments.value_of("input").unwrap();
    let output = arguments.value_of("output").unwrap();
    let format = arguments.value_of("format").unwrap();

    ////////////////////
    // Extra Handling
    ////////////////////

    let format = match format {
        "png" => ImageFormat::PNG,
        "jpeg" => ImageFormat::JPEG,
        "gif" => ImageFormat::GIF,
        // "webp" => ImageFormat::WEBP, // no encoder now
        "ppm" => ImageFormat::PPM,
        // "tiff" => ImageFormat::TIFF, // no encoder now
        // "tga" => ImageFormat::TGA,   // no encoder now
        // "bmp" => ImageFormat::BMP,   // no encoder now
        "ico" => ImageFormat::ICO,
        // "hdr" => ImageFormat::HDR,   // waiting for new release
        _ => unreachable!(),
    };

    ////////////////////
    // Image Processing
    ////////////////////

    let img = image::open(input).unwrap();
    img.save(&mut File::create(output).unwrap(),
             format).unwrap();
}
