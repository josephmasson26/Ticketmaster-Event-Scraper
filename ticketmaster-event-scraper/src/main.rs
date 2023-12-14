use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io;

#[derive(Deserialize, Debug)]
struct Event {
    // Define the fields of the Event struct here based on the API response
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = env::var("TICKETMASTER_API_KEY").expect("TICKETMASTER_API_KEY must be set");

    let mut city_to_dma = HashMap::new();
    city_to_dma.insert("New York", "345");
    city_to_dma.insert("Los Angeles", "324");
    // Add more cities and their corresponding DMA codes here

    let mut 

    println!("Enter a city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;
    let city = city.trim();

    let dma: &&str = match city_to_dma.get(city) {
        Some(dma) => dma,
        None => return Err(Error::new(io::Error::new(io::ErrorKind::Other, "City not found"))),
    };

    let url = format!("https://app.ticketmaster.com/discovery/v2/events.json?dmaId={}&apikey={}", dma, api_key);

    let response = reqwest::get(&url).await?;
    let events: Vec<Event> = response.json().await?;

    for event in events {
        println!("{:#?}", event);
    }

    Ok(())
}