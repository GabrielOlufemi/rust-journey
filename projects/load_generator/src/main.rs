// module decalrations
mod workers;
mod stats;
mod report;

use std::time::{Duration, Instant};
// use std::sync::Arc;
// use std::sync::atomic::{Ordering, AtomicU32};

use tokio::sync::mpsc;
use tokio::time::interval;

use report::print_report;
use workers::{RequestOutcome, send_request};
use stats::{summarise};
use reqwest::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build()?;

    let url = "http://127.0.0.1:8002/";

    let start = Instant::now();
    let outcomes = spawn_requests(&client, url, 100, Duration::from_secs(5)).await;
    let wall_clock = start.elapsed();

    let summary = summarise(&outcomes, wall_clock);

    print_report(&summary);
    
    Ok(())


}

async fn spawn_requests(client: &Client, url: &str, rate: u32, duration: Duration) -> Vec<RequestOutcome>{

    // deprecated sort of like a "shared ticket counter" - based off my understanding sha
    // let next_ticket = Arc::new(AtomicU32::new(0));

    // deadline based off duration
    let deadline = Instant::now() + duration;

    // interval
    let period = Duration::from_secs_f64(1.0 / rate as f64);

    let mut ticker = interval(period); 
    

    //our channel, multiple senders, one receiver - our aggregator
    let (tx, mut rx) = mpsc::channel::<RequestOutcome>(100);

    
    let aggregator = tokio::spawn( async move{
        let mut outcomes = Vec::new();
        while let Some(outcome) = rx.recv().await {
            outcomes.push(outcome);
        }
        outcomes
    });

    // new dispatch loop    
    while Instant::now() < deadline {
        ticker.tick().await;

        let client = client.clone();
        let url = url.to_string();
        let tx = tx.clone();

        tokio::spawn(async move {
            let outcome = send_request(&client, &url).await;

            // sort of a "ik this returns a result but idc" 
            let _ = tx.send(outcome).await;
            
        });

        
    }
    
    // workers handle store
    // let mut handles = Vec::new();

    // for _ in 0..concurrency {
    //     let client = client.clone();
    //     let url = url.to_string();
    //     // let next_ticket = Arc::clone(&next_ticket);
    //     let tx = tx.clone();

    //     // satrt worker
    //     let handle = tokio::spawn(async move {

    //         // checks if deadline is elapsed before executing task
    //         while Instant::now() < deadline {

    //             // send request and store outcome
    //             let outcome = send_request(&client, &url).await;

    //             // send outcome to aggregator
    //             if tx.send(outcome).await.is_err() {
    //                 break;
    //             }
    //         }

    //     });

    //     handles.push(handle);
    // }

    drop(tx);

    aggregator.await.expect("aggregator task panicked")
    
}









