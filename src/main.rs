extern crate skim;
use base64::encode;
use reqwest::Client;
use skim::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

async fn call_api(&self, path: String, json_map: HashMap<&str, String>) -> (u16, String) {
    let url = String::from("http://all.api.radio-browser.info/json/countries");
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
pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}
