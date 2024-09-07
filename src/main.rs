#![allow(unused)]

mod api; 
// TODO: edit interface 
// TODO: get input via cli using clap? 
fn main() {
    let query = "cebu".to_string(); 
    match api::get_weather(query) {
        Ok(weather) => {
            println!("Temperature in {} is {} C", weather.location.name, weather.current.temp_c); 
            println!("It is {}", weather.current.condition.text); 
            println!("Img url: {}", weather.current.condition.icon); 
        }
        Err(e) => {
            eprintln!("Error: {}", e); 
        }
    }
}
