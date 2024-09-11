use image::{DynamicImage, ImageFormat};
use termimage::{ops, util, AnsiOutputFormat, Options}; 
use std::{io::stdout, path::PathBuf}; 
use term_size; 

use std::fs::File;
use std::io::copy;

fn parse_url(path: String) -> String {
    format!("https:{}", path)
}

pub fn retrieve_image(path: String) -> Result<(), Box<dyn std::error::Error>>{
    let url = parse_url(path);  

    let filename = "src/weather_icon.png".to_string(); 

    let mut response = reqwest::blocking::get(url)
        .expect("Failed to get weather icon");
    let mut file = File::create(filename).expect("Failed to create file");
    copy(&mut response, &mut file).expect("Failed to save image"); 
    
    Ok(())
}

pub fn display_image(path: String) {
    let (_w, _h) = term_size::dimensions().unwrap(); 
    let (w, h) = (_w as u32, _h as u32); 

    let opts = Options {
        image: (String::from(path.clone()), PathBuf::from(path)),
        size: (w, h),
        preserve_aspect: true,
        ansi_out: Some(AnsiOutputFormat::Truecolor),
    };

    let format = ops::guess_format(&opts.image)
        .expect("Unable to extract format");

    let img = ops::load_image(&opts.image, format)
        .expect("Unable to load image");

    let dimensions = (img.width(), img.height()); 
    let img_s = ops::image_resized_size(dimensions, opts.size, opts.preserve_aspect);
    let resized = ops::resize_image(&img, img_s); 

    ops::write_ansi_truecolor(&mut stdout(), &resized);
    // ops::write_ansi(&mut stdout(), &resized, &util::ANSI_COLOURS_BLACK_BG)
    // ops::write_no_ansi(&resized)
}