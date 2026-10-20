// module decalrations
mod workers;
mod stats;
mod report;

use std::time::{Duration, Instant};
// use std::sync::atomic::{Ordering, AtomicU32};

use std::sync::Arc;
use tokio::sync::Semaphore;

use tokio::sync::mpsc;
use tokio::time::interval;

use report::print_report;
use workers::{RequestOutcome, send_request};
use stats::{summarise};
use reqwest::Client;

// cli stuff
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, ValueEnum)]
enum Mode {
    Open,
    Closed,
}

#[derive(Parser)]
#[command(about = "A small HTTP load generator")]
struct Args {
    // default_value gives the flag a fallback, so it becomes optional.
    #[arg(long, default_value = "http://127.0.0.1:8002/")]
    url: String,

    // Open mode: requests per second. Closed mode: the number of workers.
    #[arg(long, default_value_t = 20)]
    rate: u32,

    // Seconds the run lasts.
    #[arg(long, default_value_t = 20)]
    duration: u64,

    #[arg(long, value_enum, default_value_t = Mode::Open)]
    mode: Mode,

    // Closed mode only: how many workers send in parallel.
    #[arg(long, default_value_t = 1)]
    workers: u32,
}

// -- end of cli stuff 


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build()?;

    // let url = "http://127.0.0.1:8002/";

    // let start = Instant::now();
    // let outcomes = spawn_requests(&client, url, 20, Duration::from_secs(20)).await;    
    // let wall_clock = start.elapsed();

    let args = Args::parse(); 
    let duration = Duration::from_secs(args.duration);

    let start = Instant::now();

    let outcomes = match args.mode {
        Mode::Open => spawn_requests(&client, &args.url, args.rate, duration).await,
        Mode::Closed => spawn_closed_loop(&client, &args.url, args.workers, duration).await,
    };

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
    let (tx, mut rx) = mpsc::channel::<RequestOutcome>(200);

    
    let aggregator = tokio::spawn( async move{
        let mut outcomes = Vec::new();
        while let Some(outcome) = rx.recv().await {
            outcomes.push(outcome);
        }
        outcomes
    });

    let semaphore = Arc::new(Semaphore::new(200));

    let mut skipped_requests: u64 = 0;

    

    // new dispatch loop    
    while Instant::now() < deadline {
        
        ticker.tick().await;

        match semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                let client = client.clone();
                let url = url.to_string();
                let tx = tx.clone();
        
                tokio::spawn(async move {
                    
                    let _permit = permit;
                    let outcome = send_request(&client, &url).await;
        
                    // sort of a "ik this returns a result but idc" 
                    let _ = tx.send(outcome).await;
                    
                });
            }

            Err(_) => {
                skipped_requests += 1; 
            }
        }
        
    }

    println!("skipped (cap reached): {skipped_requests}");
    
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


// Closed-loop: a fixed number of workers, each sends its next request only - ai generated cuz i was lazy to re-write
// after the previous one has finished. The server's speed sets the pace.
async fn spawn_closed_loop(client: &Client, url: &str, workers: u32, duration: Duration) -> Vec<RequestOutcome> {

    // Same stop condition as before: every worker quits once this time passes.
    let deadline = Instant::now() + duration;

    // Same collection setup as open-loop: many senders, one aggregator.
    let (tx, mut rx) = mpsc::channel::<RequestOutcome>(200);

    let aggregator = tokio::spawn(async move {
        let mut outcomes = Vec::new();
        while let Some(outcome) = rx.recv().await {
            outcomes.push(outcome);
        }
        outcomes
    });

    for _ in 0..workers {
        // Each worker needs its own copies, since the task must own what it uses.
        let client = client.clone();
        let url = url.to_string();
        let tx = tx.clone();

        tokio::spawn(async move {
            // THIS is what makes it closed-loop: the loop body awaits the
            // response before the next iteration begins. While the server is
            // frozen, this worker sends nothing.
            while Instant::now() < deadline {
                let outcome = send_request(&client, &url).await;

                // If the aggregator is gone, there's nobody to report to: stop.
                if tx.send(outcome).await.is_err() {
                    break;
                }
            }
            // The worker's tx clone is dropped here as the task ends.
        });
    }

    // Drop our original sender. The channel closes once every worker's clone is
    // dropped too, which happens when each worker passes the deadline and finishes
    // its last in-flight request. So we don't need to keep the join handles:
    // awaiting the aggregator already waits for all the workers.
    drop(tx);

    aggregator.await.expect("aggregator task panicked")
}






