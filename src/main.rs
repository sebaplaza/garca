extern crate skim;
use skim::prelude::*;
use std::io::Cursor;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod api;
mod player;

async fn choose() -> String {
    let countries = api::get_countries().await.unwrap();
    let country = skim_show(countries);
    println!("selected country: {}", country);
    let stations = api::get_stations(&country).await.unwrap();
    let station = skim_show(stations);
    let station: Vec<&str> = station.split('|').collect();
    let station_name = station[0];
    println!("selected station: {}", station_name);
    let station_url = station[1].trim();
    println!("selected station url: {}", station_url);
    station_url.to_string()
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
async fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let home = termion::cursor::Goto(1, 1);
    let clear = termion::clear::All;
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}q to exit, r to listen radio"#, home, clear).unwrap();
    stdout.flush().unwrap();
    let mut my_player: Option<player::Player> = None;
    //detecting keydown events
    for key in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(stdout, "{}{}", home, clear).unwrap();
        //i reckon this speaks for itself
        match key.unwrap() {
            Key::Char('f') => println!("Marked as favorite!"),
            Key::Char('q') => {
                if let Some(value) = my_player {
                    value.stop();
                }
                break;
            }
            Key::Char('r') => {
                if let Some(value) = my_player {
                    value.stop();
                }
                let station_url = choose().await;
                let value = player::Player::new(station_url);
                let value = value.play();
                my_player = Some(value);
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
}
