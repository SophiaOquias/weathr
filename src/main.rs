#![allow(unused)]

mod api; 
use text_to_ascii_art::to_art; 
use clap::Parser; 

#[derive(Parser, Debug)]
#[command(
    name = "weathr", 
    author = "Sophia <sophia.oquias@gmail.com>",
    version = "0.1", 
    about = "A simple CLI app to display temperature."
)]
struct Args {
    #[arg(help = "input your current location, otherwise your approximate location")]
    location: Option<String>, 

    #[arg(short = 'f', long = "fahrenheit", help = "display temperature in Fahrenheit")]
    fahrenheit: bool
}

fn main() {
    let args = Args::parse();

    let mut query: String; 

    match args.location {
        Some(location) => query = location,
        None => {
            let coords = api::get_coords()
                .expect("Could not get current location"); 

            query = coords.loc; 
        }
    }

     
    match api::get_weather(query) {
        Ok(weather) => {
            let mut unit: char = 'C'; 
            let mut temperature: f32 = weather.current.temp_c; 
            let mut feels_like: f32 = weather.current.feelslike_c; 

            if(args.fahrenheit) {
                unit = 'F'; 
                temperature = weather.current.temp_f;
                feels_like = weather.current.feelslike_f
            }

            match to_art(format!("{}*", temperature), "", 1, 1, 0) {
                Ok(string) => println!("{}", string),
                Err(err) => {
                    println!("{}⁰{}", temperature, unit); 
                },
            }

            println!("Feels like {}⁰{}", feels_like, unit); 

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
