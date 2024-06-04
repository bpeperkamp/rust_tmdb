use clap::Parser;
#[allow(unused_imports)]
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
#[allow(unused_imports)]
use crossterm::style::{Print, Stylize};
#[allow(unused_imports)]
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
use urlencoding::encode;
#[allow(unused_imports)]
use dialoguer::{Select, Input, theme::ColorfulTheme};

// #[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Item {
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_air_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "Find movies and series on TMDB")]
#[command(version = "1.0")]
#[command(about = "It searches for your searchterm on TMDB and returns results.")]
struct Args {
    // Title of the serie or movie to lookup
    // #[arg(short, long)]
    // title: String,

    // Media type, serie or movie
    // #[arg(short, long, default_value = "tv")]
    // media_type: String,

    // Page number of the results (20 results per page)
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

#[derive(Deserialize)]
struct Items {
    results: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let bearer_token = std::env::var("TMDB_TOKEN").expect("TMDB_TOKEN must be set.");

    // let args = Args::parse();

    let base_url = "https://api.themoviedb.org/3/search/multi";

    // let search_type = if args.media_type == "movie" {
    //     "movie"
    // } else {
    //     "tv"
    // };

    let title: String = Input::new()
        .with_prompt("Search for")
        .interact_text()
        .unwrap();

    let params = "?page=1&include_adult=false&language=en-US&page=1&query=";
    let search_term = encode(&title);

    let complete_url = format!("{base_url}{params}{search_term}");

    let response = reqwest::Client::new()
        .get(complete_url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer ") + &bearer_token)
        .send()
        .await;

    match response {
        Ok(data) => {
            let items: Vec<Item> = data.json::<Items>().await?.results;

            let mut v : Vec<String> = Vec::new();

            for item in items {
                let found_title = if item.original_name.as_ref().is_some() { item.original_name } else { item.original_title };
                let found_release_date = if item.first_air_date.as_ref().is_some() { item.first_air_date } else { item.release_date };

                let strr = format!("{} - {} - {} - {} - {}", item.media_type.unwrap(), found_title.unwrap(), found_release_date.unwrap(), item.original_language.unwrap(), item.id.to_string());

                v.push(strr);
            }

            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick an Activity:")
                .default(0)
                .items(&v)
                .interact();

            let selected_index = &index.unwrap();
            let selected_value = &v[*selected_index];
            let selected_values: Vec<&str> = selected_value.split('-').collect();
            let tmdb_id = selected_values[6].replace(" ", "");

            println!("You selected {:?}", tmdb_id);
        }
        Err(_error) => {
            println!("Error sending request, server could not be reached?");
        }
    }

    Ok(())
}