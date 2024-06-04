use crossterm::style::Stylize;
use serde::Deserialize;
use reqwest::Error;
use urlencoding::encode;
use clap::Parser;
use dotenv::dotenv;

#[allow(dead_code)]
#[derive(Deserialize)]
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
}

#[derive(Parser, Debug)]
#[command(name = "Find movies and series on TMDB")]
#[command(version = "1.0")]
#[command(about = "It searches for your searchterm on TMDB and returns results.")]
struct Args {
    /// Title of the serie or movie to lookup
    #[arg(short, long)]
    title: String,

    /// Media type, serie or movie
    #[arg(short, long, default_value = "tv")]
    media_type: String,

    /// Page number of the results (20 results per page)
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Deserialize)]
struct Items {
    results: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();

    let bearer_token = std::env::var("TMDB_TOKEN").expect("TMDB_TOKEN must be set.");

    let args = Args::parse();

    let base_url = "https://api.themoviedb.org/3/search/";
    let search_type = if args.media_type == "movie" { "movie" } else { "tv" };
    let page_number =  format!("?page=") +  &args.count.to_string();
    let params = "&include_adult=false&language=en-US&page=1&query=";
    let search_term = encode(&args.title);

    let complete_url = format!("{base_url}{search_type}{page_number}{params}{search_term}");

    let response = reqwest::Client::new()
        .get(complete_url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer ") + &bearer_token)
        .send()
        .await;

    match response {
        Ok(data) => {
            let items: Vec<Item> = data.json::<Items>().await?.results;
            
            for item in items {
                if item.original_name.is_some() {
                    let print_date = if item.first_air_date.as_ref().unwrap().is_empty() { "unknown".to_string().blue() } else { item.first_air_date.unwrap().blue() };
                    let original_language = if item.original_language.as_ref().unwrap().is_empty() { "unknown".to_string().blue() } else { item.original_language.unwrap().blue() };
                    
                    let item_title = "title:".to_string().grey();
                    let item_released = "released:".to_string().grey();
                    let item_language = "language:".to_string().grey();
                    let item_id = "tmdb_id:".to_string().grey();

                    // This can probably improve quite a lot.
                    println!("{} {} - {} {} - {} {} - {} {}", item_title, &item.original_name.unwrap().blue(), item_released, print_date, item_language, original_language, item_id, &item.id.to_string().blue())
                } else {
                    let print_date = if item.release_date.as_ref().unwrap().is_empty() { "unknown".to_string().blue() } else { item.release_date.unwrap().blue() };
                    let original_language = if item.original_language.as_ref().unwrap().is_empty() { "unknown".to_string().blue() } else { item.original_language.unwrap().blue() };
                    
                    let item_title = "title:".to_string().grey();
                    let item_released = "released:".to_string().grey();
                    let item_language = "language:".to_string().grey();
                    let item_id = "tmdb_id:".to_string().grey();

                    // This can probably improve quite a lot.
                    println!("{} {} - {} {} - {} {} - {} {}", item_title, &item.original_title.unwrap().blue(), item_released, print_date, item_language, original_language, item_id, &item.id.to_string().blue())
                }
            }
        },
        Err(_error) => {
            println!("Error sending request, server could not be reached?");
        },
    }

    Ok(())
}