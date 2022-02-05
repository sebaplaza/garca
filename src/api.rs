extern crate skim;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

pub async fn get_countries() -> Result<String> {
    let url = String::from("http://all.api.radio-browser.info/json/countries");

    let (_status, data) = call_api(url).await;

    let countries: Vec<Country> = serde_json::from_str(&data)?;
    let mut str: String = String::from("");
    for country in countries {
        let countryln = format!("{}\n", &country.name);
        str.push_str(&countryln);
    }
    Ok(str)
}

pub async fn get_stations(country: &String) -> Result<String> {
    let url = format!(
        "http://all.api.radio-browser.info/json/stations/bycountryexact/{}",
        country
    );
    let (_status, data) = call_api(url).await;

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
