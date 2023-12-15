use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io;

#[derive(Deserialize, Debug)]
struct Event {
    name: String,
    dates: Dates,
    _embedded: Embedded,
}

#[derive(Deserialize, Debug)]
struct Dates {
    start: Start,
}

#[derive(Deserialize, Debug)]
struct Start {
    localDate: String,
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
    let events: Vec<Event> = response.json().await?;

    for event in events {
        println!("Event Name: {}", event.name);
        println!("Date: {}", event.dates.start.localDate);
        println!("Venue: {}", event._embedded.venues[0].name);
    }

    Ok(())
}