extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)] //Automatically generates code to allow the Weather struct to be deserialized from JSON and formatted for debugging.
struct ResultData {
    date: String,  //Date
    jahr: String,  //Year
    gre000d0: f64, //Global radiation in W/m2
    hto000d0: f64, //Amount of snow in cm
    nto000d0: f64, //Total cloud over in %
    prestad0: f64, //Air pressure in hPa
    rre150d0: f64, //Precipitation in mm
    sre000d0: f64, //Sunshine duration in min
    tre200d0: f64, //Average temperature in °C
    tre200dn: f64, //Minimal temperature in °C
    tre200dx: f64, //Maximal temperature in °C
    ure200d0: f64, //Relative humidity in %
}
#[derive(Debug, Deserialize)]
struct ApiResponse {
    total_count: usize,
    results: Vec<ResultData>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://data.bs.ch/api/explore/v2.1/catalog/datasets/100254/records?order_by=date%20DESC&limit=1");

    let json_response = reqwest::blocking::get(&url)?.text()?;

    // Deserialize JSON response into ApiResponse struct
    let api_response: ApiResponse = serde_json::from_str(&json_response)?;

    if let Some(result) = api_response.results.get(0) {
        println!("Date: {}", result.date);
        println!("Year: {}", result.jahr);
        println!("Global Radiation: {} W/m2", result.gre000d0);
        println!("Snow Amount: {} cm", result.hto000d0);
        println!("Total Cloud Cover: {} %", result.nto000d0);
        println!("Air Pressure: {} hPa", result.prestad0);
        println!("Precipitation: {} mm", result.rre150d0);
        println!("Sunshine Duration: {} min", result.sre000d0);
        println!("Average Temperature: {} °C", result.tre200d0);
        println!("Minimal Temperature: {} °C", result.tre200dn);
        println!("Maximal Temperature: {} °C", result.tre200dx);
        println!("Relative Humidity: {} %", result.ure200d0);
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    Ok(())
}