use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let result = send_request(&client, "http://127.0.0.1:8000/").await;

    println!("{:?}", result);
    println!("success? {}", result.outcome.is_success());

    Ok(())
}

async fn send_request(client: &reqwest::Client, url: &str) -> RequestOutcome {
    let start = Instant::now();

    let outcome = match client.get(url).send().await {
        Ok(response) => Outcome::Response {
            status_code: response.status().as_u16(),
        },
        Err(e) if e.is_timeout() => Outcome::Timeout,
        Err(e) => Outcome::ConnectionFailed {
            reason: e.to_string(),
        },
    };

    RequestOutcome {
        latency: start.elapsed(),
        outcome,
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Response { status_code: u16 },
    Timeout,
    ConnectionFailed { reason: String },
}

#[derive(Debug, Clone)]
struct RequestOutcome {
    latency: Duration,
    outcome: Outcome,
}

impl Outcome {
    // Only 2xx responses count as success. A 404 or 500 means the setup
    // is wrong or the server is struggling, and both are worth surfacing.
    fn is_success(&self) -> bool {
        match self {
            Outcome::Response { status_code } => (200..=299).contains(status_code),
            Outcome::Timeout => false,
            Outcome::ConnectionFailed { .. } => false,
        }
    }
}