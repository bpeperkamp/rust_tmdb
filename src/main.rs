use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

// #[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Item {
    id: u32,
    original_name: Option<String>,
    original_title: Option<String>,
    first_air_date: Option<String>,
    release_date: Option<String>,
    original_language: Option<String>,
    media_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    media_type: String,
    tmdb_id: u64
}

#[derive(Parser, Debug)]
#[command(name = "Find movies and series on TMDB")]
#[command(version = "1.0")]
#[command(about = "It searches for your searchterm on TMDB and returns results. Once selected, it sends the data to your API server.")]
struct Args {
    /// Title of the serie or movie to lookup
    #[arg(short, long)]
    title: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Items {
    results: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Get the env file
    dotenv().ok();

    let args = Args::parse();

    let bearer_token = std::env::var("TMDB_TOKEN").expect("TMDB_TOKEN must be set.");

    let base_url = "https://api.themoviedb.org/3/search/multi";

    // Get the title from terminal arguments or ask for them
    let title: String = if args.title.is_some() {
        args.title.unwrap()
    } else {
        Input::new()
            .with_prompt("Search for")
            .interact_text()
            .unwrap()
    };

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

            println!("{:?}", &items);

            let mut v: Vec<String> = Vec::new();

            for item in items {
                if item.media_type.as_ref().unwrap() != "person" {
                    
                    let found_title = if item.original_name.as_ref().is_some() {
                        item.original_name
                    } else {
                        item.original_title
                    };

                    // Convert this to a match, this code reeks of...
                    let found_release_date = if item.first_air_date.as_ref().is_some() && item.first_air_date.as_ref().unwrap().is_empty() {
                        String::from("unknown")
                    } else if item.release_date.as_ref().is_some() && item.release_date.as_ref().unwrap().is_empty() {
                        String::from("unknown")
                    } else if item.first_air_date.as_ref().is_some() {
                        item.first_air_date.unwrap()
                    } else if item.release_date.as_ref().is_some() {
                        item.release_date.unwrap()
                    } else {
                        String::from("unknown")
                    };
    
                    let separator = "»";
    
                    let strr = format!(
                        "{} {} {} {} {} {} {} {} {}",
                        item.media_type.unwrap().to_uppercase(),
                        separator,
                        found_title.unwrap(),
                        separator,
                        found_release_date,
                        separator,
                        item.original_language.unwrap_or(String::from("unknown")),
                        separator,
                        item.id.to_string()
                    );
    
                    v.push(strr.to_string());
                }
            }

            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select your choice:")
                .default(0)
                .items(&v)
                .interact();

            let selected_index = &index.unwrap();
            let selected_value = &v[*selected_index];
            let selected_values: Vec<&str> = selected_value.split("»").collect();

            let tmdb_id = selected_values[4].trim_start();

            let api_url = std::env::var("API_URL").expect("API_URL must be set.");
            let api_bearer_token = std::env::var("API_TOKEN").expect("API_TOKEN must be set.");

            let data = Data {
                media_type: String::from(selected_values[0]).to_lowercase(),
                tmdb_id: tmdb_id.to_string().parse::<u64>().unwrap()
            };

            let api_response = reqwest::Client::new()
                .post(api_url)
                .json(&data)
                .header("Accept", "application/json")
                .header("Authorization", format!("Bearer ") + &api_bearer_token)
                .send()
                .await;

            match api_response {
                Ok(_data) => {
                    println!("Data stored, check your list online!");
                }
                Err(_error) => {
                    println!("Error sending request, server could not be reached?");
                }
            }
        }
        Err(_error) => {
            println!("Error sending request, server could not be reached?");
        }
    }

    Ok(())
}
