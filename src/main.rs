#![allow(unused)]

mod api; 
mod utils; 
// TODO: edit interface 
use text_to_ascii_art::to_art; 

// TODO: get input via cli using clap? 
fn main() {
    let query = "cebu".to_string(); 
    match api::get_weather(query) {
        Ok(weather) => {
            match to_art(format!("{}*", weather.current.temp_c), "", 0, 0, 0) {
                Ok(string) => println!("{}", string),
                Err(err) => {
                    println!("{}⁰", weather.current.temp_c); 
                },
            }

            println!("{}, {}, {}", 
                weather.location.name, 
                weather.location.region, 
                weather.location.country
            ); 
            println!("{}", weather.current.condition.text); 

            // too slow and image has borders that i can't get rid of 
            // utils::retrieve_image(weather.current.condition.icon); 
            // utils::display_image("src/weather_icon.png".to_string()); 
        }
        Err(e) => {
            eprintln!("Error: {}", e); 
        }
    }
}
