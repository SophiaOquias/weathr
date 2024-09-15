use reqwest;
use serde::Deserialize;
use dotenvy::dotenv; 
use std::env; 

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
    pub feelslike_c: f32, 
    pub feelslike_f: f32,
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String, 
    pub icon: String, 
    pub code: i32, 
}

#[derive(Deserialize)]
pub struct Coordinates {
    pub loc: String,
}

pub fn get_coords() -> Result<Coordinates, Box<dyn std::error::Error>> {
    dotenv().ok(); 
    let api_key = env::var("IPINFO_API").expect("API key for ipinfo.io not found"); 
    let url = format!("http://ipinfo.io?token={}", api_key);
    
    let resp: Coordinates = reqwest::blocking::get(url)?.json()?;
    
    Ok(resp)
}

pub fn get_weather(query: String) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    dotenv().ok(); 
    let api_key = env::var("WEATHER_API").expect("API key for weatherapi.com not found");
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            api_key, 
            query
    );

    let resp: WeatherResponse = reqwest::blocking::get(url)?
        .json()?;

    Ok(resp) 
}