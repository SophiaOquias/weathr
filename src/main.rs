#![allow(unused)]

mod api; 
use text_to_ascii_art::to_art; 
// TODO: get input via cli using clap? 
// TODO: get current location 
// TODO: accept argument for temp unit f or c, default in c 
fn main() {
    // let query = get_coords()
    //     .expect("Could not get current location");  
    match api::get_weather("lahug".to_string()) {
        Ok(weather) => {
            match to_art(format!("{}*", weather.current.temp_c), "", 1, 0, 0) {
                Ok(string) => println!("{}", string),
                Err(err) => {
                    println!("{}⁰", weather.current.temp_c); 
                },
            }

            println!("Feels like {}⁰", weather.current.feelslike_c); 

            println!("{}, {}, {}", 
                weather.location.name, 
                weather.location.region, 
                weather.location.country
            ); 
            println!("{}", weather.current.condition.text); 
        }
        Err(e) => {
            eprintln!("Error: {}", e); 
        }
    }
}
