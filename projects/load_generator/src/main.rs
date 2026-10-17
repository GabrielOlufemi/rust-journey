// module decalrations
mod workers;
mod stats;
mod report;

use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU32};

use tokio::sync::mpsc;

use report::print_report;
use workers::{RequestOutcome, send_request};
use stats::{summarise};
use reqwest::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build()?;

    let url = "http://127.0.0.1:8002/";


    let start = Instant::now();
    let outcomes = spawn_requests(&client, url, 300, 50).await;
    let wall_clock = start.elapsed();

    let summary = summarise(&outcomes, wall_clock);

    print_report(&summary);
    
    
    Ok(())


}

async fn spawn_requests(client: &Client, url: &str, count: u32, concurrency: u32) -> Vec<RequestOutcome>{

    // sort of like a "shared ticket counter" - based off my understanding sha
     let next_ticket = Arc::new(AtomicU32::new(0));

    //our channel, multiple senders, one receiver - our aggregator
    let (tx, mut rx) = mpsc::channel::<RequestOutcome>(100);

    
    let aggregator = tokio::spawn( async move{

        let mut outcomes = Vec::new();

        while let Some(outcome) = rx.recv().await {
            outcomes.push(outcome);
        }

        outcomes
    });
    
    // workers handle store
    let mut handles = Vec::new();

    // start tasks and store handles to those tasks
    for _ in 0..concurrency {
        let client = client.clone();
        let url = url.to_string();
        let next_ticket = Arc::clone(&next_ticket);
        let tx = tx.clone();

        // satrt worker
        let handle = tokio::spawn( async move {

            loop {
                let ticket = next_ticket.fetch_add(1, Ordering::Relaxed);

                if ticket >= count {
                    break;
                }

                // send request and store outcome
                let outcome = send_request(&client, &url).await;

                // send outcome to aggregator
                if tx.send(outcome).await.is_err() {
                    break;
                }
            }

        });

        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        // 
        handle.await.expect("worker task panicked");
    }


    aggregator.await.expect("aggregator task panicked")
    
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








