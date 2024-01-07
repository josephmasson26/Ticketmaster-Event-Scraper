# Ticketmaster Event Scraper
This is a Rust Command-Line application built with Serde to make calls to Ticketmaster's Discovery API and return concerts by city using DMA codes.

The app returns the top 20 concerts by DMA/city with their dates and times.

## API Key
You need a Ticketmaster API key to run the program, which can be obtained for free from:
https://developer.ticketmaster.com/products-and-docs/apis/getting-started/

## Installation
1. Clone the repository:
```
  git clone josephmasson26/Ticketmaster-Event-Scraper
```
2. Navigate to the directory and enter the build's folder
```
  cd ticketmaster-event-scraper
```
3. To set the API key envrionment variable, enter the command:
- For MacOS/Linux:
```
  export TICKETMASTER_API_KEY="your_api_key"
```
- For Windows:
```
  set TICKETMASTER_API_KEY="your_api_key"
```
4. In the directory's terminal, run the build command.
```
  cargo run
```

## Using the Scraper
- When prompted, enter the city you want to see events for!
- The program uses a hashmap to convert cities to DMA codes for the request
- The cities' DMA codes are here: https://developer.ticketmaster.com/products-and-docs/apis/discovery-api/v2/#supported-dma

## Known Issues
- Although they have DMA codes, the Discovery API is not functioning correctly for international DMA codes outside the U.S.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- This project was developed to make it easier to access international concerts without using other language Ticketmaster domains
- I hope to update the project to fix the issues with accessing international events!

### Ticketmaster Event Scraper - Developed Joseph Masson 2023
  
