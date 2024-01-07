use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io;


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
}

#[derive(Debug, Deserialize)]
struct Image {
    ratio: String,
    url: String,
    width: u32,
    height: u32,
    fallback: bool,
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
    io::stdin().read_line(&mut city).expect("Failed to input city");
    let city = city.trim();

    let dma: &&str = match city_to_dma.get(city) {
        Some(dma)=>dma,
        None => todo!(), 
        //todo is a macro for unimplemented!()
    };

    let url = format!("https://app.ticketmaster.com/discovery/v2/events.json?dmaId={}&apikey={}", dma, api_key);

    let response = reqwest::get(&url).await?;


    let response_body = response.text().await?;
    std::fs::write("response.json", response_body.clone()).expect("Unable to write file");

    let api_response: ApiResponse = serde_json::from_str(&response_body).expect("Unable to deserialize response");

    if let Some(embedded) = api_response._embedded {
        for event in embedded.events {
            println!("Event Name: {}", event.name);
        }
    } else {
        println!("No events found.");
    }

    Ok(())
}