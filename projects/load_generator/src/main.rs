// module decalrations
mod workers;
mod stats;

use std::time::{Duration, Instant};
use workers::{RequestOutcome, send_request};
use stats::{summarise};
use reqwest::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build()?;

    let url = "http://127.0.0.1:8001/";


    let start = Instant::now();
    let outcomes = spawn_requests(&client, url, 300).await;
    let wall_clock = start.elapsed();

    let summary = summarise(&outcomes, wall_clock);

    println!("{:#?}", summary);
    Ok(())


}

async fn spawn_requests(client: &Client, url: &str, count: u32) -> Vec<RequestOutcome>{

    let mut handles = Vec::new();

    // start tasks and store handles to those tasks
    for _ in 0..count {
        let client = client.clone();
        let url = url.to_string();

        let handle = tokio::spawn( async move {
            send_request(&client, &url).await
        });

        handles.push(handle);
    }

    // wait for tasks and collect results

    let mut outcomes = Vec::new();

    for handle in handles {
        let outcome = handle.await.expect("request task panicked");
        outcomes.push(outcome);
    }

    outcomes


}

async fn spam_requests(client: &reqwest::Client, url: &str, count: &u32) -> Vec<RequestOutcome> {

    let mut i = 1;
    let mut outcomes = Vec::new();

    while &i <= count {

        let outcome = send_request(&client, &url).await;

        println!("request {i}: {:?}", outcome.latency);
        
        outcomes.push(outcome);
        i+= 1;
    }

    outcomes

}








