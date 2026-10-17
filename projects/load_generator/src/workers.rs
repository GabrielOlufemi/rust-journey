use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum Outcome {
    Response { status_code: u16 },
    Timeout,
    ConnectionFailed { reason: String }
}

#[derive(Debug,Clone)]
pub struct RequestOutcome {
    pub latency: Duration,
    pub outcome: Outcome 
}

impl Outcome { 
    pub fn is_success(&self) -> bool {
        match self {
            Outcome::Response { status_code } => (200..=299).contains(status_code),
            Outcome::Timeout => false,
            Outcome::ConnectionFailed { .. } => false
        }
    }
}


pub async fn send_request(client: &reqwest::Client, url: &str) -> RequestOutcome {

    let start = Instant::now();

    let outcome = match client.get(url).send().await {
        Ok(response) => Outcome::Response { status_code: response.status().as_u16() },
        Err(e) if e.is_timeout() => Outcome::Timeout,
        Err(e) => Outcome::ConnectionFailed { reason: e.to_string() }
    };

    RequestOutcome {
        latency : start.elapsed(),
        outcome : outcome
    }
}
