use reqwest;
use serde::Deserialize;

use image::{DynamicImage, ImageFormat};
use termimage::{ops, util, AnsiOutputFormat, Options}; 
use std::{io::stdout, path::PathBuf}; 
use term_size; 

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub location: Location, 
    pub current: Current,
}

#[derive(Deserialize)]
pub struct Location {
    pub name: String, 
    pub region: String, 
    pub country: String, 
}

#[derive(Deserialize)]
pub struct Current {
    pub temp_c: f32, 
    pub temp_f: f32, 
    pub condition: Condition, 
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String, 
    pub icon: String, 
    pub code: i32, 
}

pub fn get_weather(query: String) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let api_key = "76436078e0454ee8bd9131745240609".to_string();
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
            api_key, 
            query
    );

    let resp: WeatherResponse = reqwest::blocking::get(url)?
        .json()?;

    Ok(resp) 
}

fn display_image(path: String) {
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
    println!("image dimensions: {:?}", dimensions); 
    let img_s = ops::image_resized_size(dimensions, opts.size, opts.preserve_aspect);
    let resized = ops::resize_image(&img, img_s); 

    ops::write_ansi_truecolor(&mut stdout(), &resized);
}