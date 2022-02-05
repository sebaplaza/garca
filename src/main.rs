extern crate skim;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use skim::prelude::*;
use std::io::Cursor;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
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

    let (_status, data) = call_api(url).await;

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
async fn choose() -> Result<std::process::Child> {
    let countries = get_countries().await?;
    let country = skim_show(countries);
    println!("selected country: {}", country);
    let stations = get_stations(&country).await?;
    let station = skim_show(stations);
    let station: Vec<&str> = station.split('|').collect();
    let station_name = station[0];
    println!("selected station: {}", station_name);
    let station_url = station[1].trim();
    println!("selected station url: {}", station_url);
    println!("Listening...");
    let child = Command::new("mpv")
        .arg(station_url)
        .spawn()
        .expect("failed to execute process");
    Ok(child)
}

fn kill_process(child: Option<std::process::Child>) {
    if let Some(mut value) = child {
        value.kill();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let home = termion::cursor::Goto(1, 1);
    let clear = termion::clear::All;
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}q to exit, r to listen radio"#, home, clear).unwrap();
    stdout.flush().unwrap();
    //detecting keydown events
    let mut child: Option<std::process::Child> = None;
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(stdout, "{}{}", home, clear).unwrap();
        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Char('h') => println!("Hello world!"),
            Key::Char('q') => {
                kill_process(child);
                break;
            }
            Key::Char('r') => {
                kill_process(child);
                child = Some(choose().await?);
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
    Ok(())
}
