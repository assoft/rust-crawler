// #![allow(dead_code)]
// #![allow(unused)]

extern crate scraper;

use std::{collections::HashMap};
use std::iter::FromIterator;

use scraper::{Html, Selector};
use serde::{Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use serde_json;

#[derive(Debug, Serialize)]
struct Quake {
    date: String,
    latitude: String,
    longitude: String,
    depth: String,
    md: String,
    ml: String,
    mw: String,
    location: String,
    info: String
}

fn main() {
    let prayers = fetch_today_prayer_times();
    match prayers {
        Ok(data) => {println!("{:?}", data)}
        Err(e) => { println!("Hata: {:?}", e) }
    }

    let quakes = fetch_latest_quakes();
    match quakes {
        Ok(data) => { println!("{:?}", data); }
        Err(e) => { println!("Hata: {:?}", e); }
    }
}   

#[tokio::main]
async fn fetch_today_prayer_times () -> Result<HashMap<String, String>, reqwest::Error> {
    let doc = reqwest::Client::new()
        .get("https://namazvakitleri.diyanet.gov.tr/tr-TR/9303")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    let query = Html::parse_fragment(&doc);

    let wrapper = Selector::parse(".today-pray-times").unwrap();
    let selector_row = Selector::parse(".w3-col").unwrap();
    let selector_title = Selector::parse(".tpt-title").unwrap();
    let selector_time = Selector::parse(".tpt-time").unwrap();

    let rows = query.select(&wrapper).next().unwrap();

    let mut prayer_data: HashMap<String, String> = HashMap::new();

    for el in rows.select(&selector_row) {
        let title = el.select(&selector_title).next().unwrap();
        let time = el.select(&selector_time).next().unwrap();

        let titles = title.text().collect::<Vec<_>>();
        let times = time.text().collect::<Vec<_>>();

        prayer_data.insert(titles.into_iter().collect::<String>(), times.into_iter().collect::<String>());
    }
    // println!("{:?}", prayer_data);
    Ok(prayer_data)
}


#[tokio::main]
async fn fetch_latest_quakes () -> Result<Vec<Quake>, reqwest::Error> {
    let doc = reqwest::Client::new()
        .get("http://www.koeri.boun.edu.tr/scripts/lst6.asp")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
        .send()
        .await?
        .text_with_charset("windows-1254")
        .await?;

    let query = Html::parse_fragment(&doc);

    let wrapper = Selector::parse("pre").unwrap();
    let mut quakes_data: Vec<Quake> = Vec::new();

    let table = query.select(&wrapper).last().unwrap().inner_html();

    let mut table_split = Vec::from_iter(table.lines().map(String::from));
    
    table_split.drain(0..6);

    table_split.iter().for_each(|row| {
        let row_data: Vec<String> = row.split("  ")
            .filter(|&x| !x.is_empty())
            .map(|s| {
                String::from_utf8_lossy(s.trim().as_bytes()).to_string()
            })
            .collect();

        if row_data.len() == 9 {
            let quake = Quake {
                date: row_data[0].to_string(),
                latitude: row_data[1].to_string(),
                longitude: row_data[2].to_string(),
                depth: row_data[3].to_string(),
                md: row_data[4].to_string(),
                ml: row_data[5].to_string(),
                mw: row_data[6].to_string(),
                location: row_data[7].to_string(),
                info: row_data[8].to_string(),
            };
            quakes_data.push(quake);
        }
    });
    
    let mut file = BufWriter::new(File::create("quakes_data.json").unwrap());
    let json = serde_json::to_string_pretty(&quakes_data).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    Ok(quakes_data)
}