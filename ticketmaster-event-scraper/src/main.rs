// Ticketmaster API Documentation: https://developer.ticketmaster.com/products-and-docs/apis/getting-started/
// Ticketmaster API Explorer: https://developer.ticketmaster.com/products-and-docs/apis/discovery-api/v2/#search-events-v2
// Supported DMA: https://developer.ticketmaster.com/products-and-docs/apis/discovery-api/v2/#supported-dma
// Developed Joseph Masson 2024

//This is a simple program that takes a city as input and returns a list of music events in that city from 
//Ticketmaster's Discovery API. The program uses the DMA ID of the city to make the API call with a HashMap.

//I built this project to practice RUST and API usage!

//To run this program, you must have a Ticketmaster API key. 
//You can get one here: https://developer.ticketmaster.com/products-and-docs/apis/getting-started/


//Program Code:

// Rust has several warnings for camel case variables, which are part of the JSON response from the API.
// I disable these warnings for ease of use
#![allow(warnings)]


use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::io;

// Include the hashmap structure from dma.rs
mod dma;

// These structs break up the JSON response into parts for easy deserialization
#[derive(Debug, Deserialize)]
struct ApiResponse {
    _embedded: Option<Embedded>,
}

#[derive(Debug, Deserialize)]
struct Embedded {
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Event {
    name: String,
    #[serde(rename = "type")]
    event_type: String,
    id: String,
    test: bool,
    url: String,
    locale: String,
    images: Vec<Image>,
    dates: Dates,
}

#[derive(Debug, Deserialize)]
struct Image {
    ratio: Option<String>,
    url: String,
    width: u32,
    height: u32,
    fallback: bool,
}

#[derive(Debug, Deserialize)]
struct Dates {
    start: Start,
    timezone: Option<String>,
    status: Status,
    spanMultipleDays: bool,
}

#[derive(Debug, Deserialize)]
struct Start {
    localDate: String,
    localTime: Option<String>,
    dateTime: Option<String>,
    dateTBD: bool,
    dateTBA: bool,
    timeTBA: bool,
    noSpecificTime: bool,
}

#[derive(Debug, Deserialize)]
struct Status {
    code: String,
}


// Main function of the program
#[tokio::main]
async fn main() -> Result<(), Error> {

    // Get the API key from the environment variables
    let api_key = env::var("TICKETMASTER_API_KEY").expect("TICKETMASTER_API_KEY must be set");

    // Get the hashmap of cities to DMA codes from dma.rs
    let city_to_dma = dma::get_hashmap();

    // Get the city from the user
    println!("Enter a city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to input city");
    let city = city.trim();

    // Get the DMA code from the hashmap
    let dma: &&str = match city_to_dma.get(city) {
        Some(dma)=>dma,
        None => todo!(), 
    };

    // Create the API call URL
    let url = format!("https://app.ticketmaster.com/discovery/v2/events.json?classificationName=music&dmaId={}&apikey={}", dma, api_key);

    // Make the API call
    let response = reqwest::get(url).await?;

    
    let response_body = response.text().await?;

    let api_response: ApiResponse = serde_json::from_str(&response_body).expect("Unable to deserialize response");

    if let Some(mut embedded) = api_response._embedded {
        embedded.events.sort_by(|a, b| a.dates.start.localDate.cmp(&b.dates.start.localDate));
        for event in embedded.events {
            println!(
                "Event Name: {}, Date: {}, Time: {}",
                event.name,
                event.dates.start.localDate,
                event.dates.start.localTime.as_ref().unwrap_or(&"Not Provided".to_string())
            );
        }
    } else {
        println!("No events found.");
    }

    Ok(())
}

//Known limitations - program only shows the top 20 events by city, and only shows music events.
//International DMA codes are not supported, which is a limitation of the Ticketmaster API not represented
//in the documentation.