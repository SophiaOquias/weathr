# weathr 
This is a simple cli app that gives simple weather information (i.e. temperature and weather condition). 

## Prerequisites 
- rust 
- your own API keys from: 
  - api.weatherapi.com
  - ipinfo.io
- a .env file with the following: 
  - ```
    WEATHER_API="<your key here>"
    IPINFO_API="<your key here>"
    ```

## How to use 
```
Usage: cargo run [OPTIONS] [LOCATION]

Arguments:
  [LOCATION]  input your current location, otherwise your approximate location

Options:
  -f, --fahrenheit  display temperature in Fahrenheit
  -h, --help        Print help
  -V, --version     Print version
```
