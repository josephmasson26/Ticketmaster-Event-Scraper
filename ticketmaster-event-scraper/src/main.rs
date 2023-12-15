use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    _embedded: EmbeddedEvents,
}

#[derive(Deserialize, Debug)]
struct EmbeddedEvents {
    events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
struct Event {
    name: String,
    dates: Dates,
    classifications: Vec<Classification>,
    _embedded: Embedded,
}

#[derive(Deserialize, Debug)]
struct Classification {
    genre: Genre,
}

#[derive(Deserialize, Debug)]
struct Genre {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Dates {
    start: LocalDate,
}

#[derive(Deserialize, Debug)]
struct LocalDate {
    local_date: String,
}

#[derive(Deserialize, Debug)]
struct Start {
    local_date: String,
}

#[derive(Deserialize, Debug)]
struct Embedded {
    venues: Vec<Venue>,
}

#[derive(Deserialize, Debug)]
struct Venue {
    name: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = env::var("TICKETMASTER_API_KEY").expect("TICKETMASTER_API_KEY must be set");

    let mut city_to_dma = HashMap::new();
    city_to_dma.insert("Barcelona", "902");
    // Add more cities and their corresponding DMA codes here
    // Add Hashmap later? Or an object of some type without 900 insert statements

    println!("Enter a city:");
    let mut city = String::new();
    let _ = io::stdin().read_line(&mut city);
    let city = city.trim();

    let dma: &&str = match city_to_dma.get(city) {
        Some(dma)=>dma,
        None => todo!(), 
        //todo is a macro for unimplemented!()
    };

    let url = format!("https://app.ticketmaster.com/discovery/v2/events.json?dmaId={}&apikey={}", dma, api_key);

    let response = reqwest::get(&url).await?;
    let api_response: ApiResponse = response.json().await?;

    for event in api_response._embedded.events {
        println!("Event Name: {}", event.name);
        println!("Event Date: {}", event.dates.start.local_date);
        println!("Venue: {}", event._embedded.venues[0].name);
    }

    Ok(())
}