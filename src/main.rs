extern crate skim;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use skim::prelude::*;
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
struct Country {
    name: String,
    stationcount: u64,
}
#[derive(Serialize, Deserialize)]
struct Station {
    name: String,
    country: String,
    url: String,
}

async fn get_countries() -> Result<String> {
    let url = String::from("http://all.api.radio-browser.info/json/countries");

    let (status, data) = call_api(url).await;

    let countries: Vec<Country> = serde_json::from_str(&data)?;
    let mut str: String = String::from("");
    for country in countries {
        let countryln = format!("{}\n", &country.name);
        str.push_str(&countryln);
    }
    Ok(str)
}

async fn get_stations(country: &String) -> Result<String> {
    let url = format!(
        "http://all.api.radio-browser.info/json/stations/bycountryexact/{}",
        country
    );
    println!("{}", url);
    let (status, data) = call_api(url).await;

    let stations: Vec<Station> = serde_json::from_str(&data)?;
    let mut str: String = String::from("");
    for station in stations {
        let stationln = format!("{} | {}\n", &station.name, &station.url);
        str.push_str(&stationln);
    }
    Ok(str)
}
async fn call_api(url: String) -> (u16, String) {
    let client = Client::new();
    let res = client
        .get(url)
        .send()
        .await
        .expect("Problem calling the radio api");
    let status = res.status().as_u16();
    let data = res.text().await.expect("Problem extracting data");
    return (status, data);
}

fn skim_show(list: String) -> String {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(list));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    selected_items[0].output().into_owned()
}

#[tokio::main]
async fn main() -> Result<()> {
    let countries = get_countries().await?;
    let country = skim_show(countries);
    println!("selected country: {}", country);
    let stations = get_stations(&country).await?;
    let station = skim_show(stations);
    let station: Vec<&str> = station.split('|').collect();
    let station_name = station[0];
    println!("selected station: {}", station_name);
    let station_name = station[1].trim();
    println!("selected station url: {}", station_name);
    Ok(())
}
