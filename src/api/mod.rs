use reqwest;
use serde::Deserialize;

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
    let api_key = "d78d6f70c265c9".to_string(); 
    let url = format!("http://ipinfo.io?token={}", api_key);
    
    let resp: Coordinates = reqwest::blocking::get(url)?.json()?;
    
    Ok(resp)
}

pub fn get_weather(query: String) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let api_key = "76436078e0454ee8bd9131745240609".to_string();
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            api_key, 
            query
    );

    let resp: WeatherResponse = reqwest::blocking::get(url)?
        .json()?;

    Ok(resp) 
}