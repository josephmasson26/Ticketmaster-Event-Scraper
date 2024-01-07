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


#![allow(warnings)]


use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::io;

mod dma;


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


#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = env::var("TICKETMASTER_API_KEY").expect("TICKETMASTER_API_KEY must be set");

    let city_to_dma = dma::get_hashmap();

    println!("Enter a city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to input city");
    let city = city.trim();

    let dma: &&str = match city_to_dma.get(city) {
        Some(dma)=>dma,
        None => todo!(), 
    };

    let url = format!("https://app.ticketmaster.com/discovery/v2/events.json?classificationName=music&dmaId={}&apikey={}", dma, api_key);

    let response = reqwest::get(url).await?;

    let response_body = response.text().await?;
    std::fs::write("response.json", response_body.clone()).expect("Unable to write file");

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