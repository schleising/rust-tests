use serde::Deserialize;
use chrono::prelude::*;
// use indicatif::ProgressIterator;

#[derive(Deserialize)]
struct Match {
    status: String,
    start_time_iso: DateTime<Local>,
    home_team: String,
    home_team_score: Option<u32>,
    away_team: String,
    away_team_score: Option<u32>,
}

#[derive(Deserialize)]
struct MatchList {
    matches: Vec<Match>,
}

fn unwrap_score(score: Option<u32>) -> String {
    match score {
        // If the score is Some, format it as a string
        Some(s) => format!("{s:1}"),
        // If the score is None, set it to 0
        None => "0".to_string(),
    }
}
/*
fn download_and_parse_matches(client: &reqwest::blocking::Client, print_matches: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Set the URL
    let url: &str = "https://schleising.net/football/api/";

    // Build the request
    let request: reqwest::blocking::RequestBuilder = client.get(url);

    // Send the request and get the response using ? to return early if there is an error
    let response = request.send()?;

    // Check the response status
    if !response.status().is_success() {
        // Return an error
        return Err(format!("Error: {}", response.status()).into());
    }

    // Parse the response as JSON using serde
    let match_list: MatchList = response.json()?;

    // Print the matches
    for m in match_list.matches {
        // Convert the scores to strings handling the case where the score is None
        let home_score: String = unwrap_score(m.home_team_score);
        let away_score: String = unwrap_score(m.away_team_score);

        // The start time has already been parsed as a DateTime<Local> so we can format it as a string directly using the format method
        let start_time: String = m.start_time_iso.format("%a %d %b %Y %H:%M").to_string();

        // Print the match if print_matches is true
        if print_matches {
            println!("{:12} {} - {} {:13} - {} - {}", m.home_team, home_score, away_score, m.away_team, start_time, m.status);
        }
    }

    // Return Ok
    Ok(())
}
*/
// Async version
async fn download_and_parse_matches_async(_client: &reqwest::Client, print_matches: bool, i: u32) -> Result<(), Box<dyn std::error::Error>> {

    println!("Iteration Start: {i}");

    // Set the URL
    let url: &str = "https://schleising.net/football/api/";

    // Build the request
    let response = reqwest::get(url).await?;

    // println!("Request: {:?}", request);

    // Send the request and get the response using await to wait for the response
    // let response = request.send().await?;

    println!("Response: {response:?}");

    // Check the response status
    if !response.status().is_success() {
        println!("Error: {}", response.status());
        // Return an error
        return Err(format!("Error: {}", response.status()).into());
    }

    // Parse the response as JSON using serde
    let match_list: MatchList = response.json().await?;

    println!("Match List");

    // Print the matches
    for m in match_list.matches {
        // Convert the scores to strings handling the case where the score is None
        let home_score: String = unwrap_score(m.home_team_score);
        let away_score: String = unwrap_score(m.away_team_score);

        // The start time has already been parsed as a DateTime<Local> so we can format it as a string directly using the format method
        let start_time: String = m.start_time_iso.format("%a %d %b %Y %H:%M").to_string();

        // Print the match if print_matches is true
        if print_matches {
            println!("{:12} {} - {} {:13} - {} - {}", m.home_team, home_score, away_score, m.away_team, start_time, m.status);
        }
    }

    println!("Iteration End: {i}");

    // Return Ok
    Ok(())
}

async fn the_async_function() {
    // // Start a timer
    // let timer = std::time::Instant::now();

    // // Create a reqwest client
    // let client: reqwest::blocking::Client = reqwest::blocking::Client::new();

    // // Call download_and_parse_matches 1000 times
    // for iteration in (0..1000).progress() {
    //     // Print the matches on the first iteration
    //     let print_matches: bool = iteration == 0;

    //     // Call download_and_parse_matches and print an error if it returns an error
    //     if let Err(e) = download_and_parse_matches(&client, print_matches) {
    //         println!("Error: {}", e);

    //         // Break out of the loop
    //         break;
    //     }
    // }

    // // Print the time taken
    // println!("Time taken: {}s", timer.elapsed().as_secs());

    // Start a timer
    let timer = std::time::Instant::now();

    // Create a reqwest client
    let client: reqwest::Client = reqwest::Client::new();

    // Create 1000 futures and wait for them to complete
    let futures: Vec<_> = (0..6).map(|i: u32| download_and_parse_matches_async(&client, true, i)).collect();

    // Wait for all the futures to complete
    futures::future::join_all(futures).await;

    // Print the time taken
    println!("Time taken: {}s", timer.elapsed().as_secs());
}

#[tokio::main]
async fn main() {
    // Call the_async_function
    futures::executor::block_on(the_async_function());
}
