use std::time::Duration;
use std::collections::BTreeMap;
use crate::workers::{Outcome, RequestOutcome};

#[derive(Debug)]
pub struct Summary {
  pub total: usize,
  pub successes: usize,
  pub failures: usize,
  pub wall_clock: Duration,
  pub mean: Duration,
  pub min: Duration,
  pub max: Duration,
  pub throughput: f64,
  pub p50: Duration,
  pub p95: Duration,
  pub p99: Duration,
  pub timeouts: usize,
  pub connection_failures: usize,
  pub failure_reasons: BTreeMap<String, usize>,
}

pub fn summarise (outcomes: &[RequestOutcome], wall_clock:Duration) -> Summary {
  let total = outcomes.len();
  let successes = outcomes.iter().filter(|o| o.outcome.is_success()).count();

  let failures = total-successes;

  // let mut total_latency = Duration::ZERO; 

  // for outcome in outcomes {
  //   total_latency += outcome.latency;
  // } 

  // outcome breakdown
  let mut timeouts = 0;
  let mut connection_failures = 0;
  let mut status_codes: BTreeMap<u16, usize> = BTreeMap::new();

  for o in outcomes {
    match &o.outcome {
      Outcome::Response {status_code} => {*status_codes.entry(*status_code).or_insert(0) += 1;}
      Outcome::Timeout => timeouts += 1,
      Outcome::ConnectionFailed { .. } => connection_failures += 1,
    }
  }

  let min: Duration = outcomes.iter().map(|o| o.latency).min().unwrap_or(Duration::ZERO);
  let max: Duration = outcomes.iter().map(|o| o.latency).max().unwrap_or(Duration::ZERO);
  
  let total_latency: Duration = outcomes.iter().map(|o| o.latency).sum();
  let mean = if total == 0 {
    Duration::ZERO
  } else {
    total_latency / total as u32
  };

  let throughput = total as f64 / wall_clock.as_secs_f64(); 

  let mut latencies: Vec<Duration> = outcomes.iter().map(|o| o.latency).collect();
  latencies.sort();
    
  let p50 = percentile(&latencies, 50.0);
  let p95 = percentile(&latencies, 95.0);
  let p99 = percentile(&latencies, 99.0);
  
  
  Summary {
    total,
    successes,
    failures,
    wall_clock,
    mean,
    min,
    max,
    throughput,
    p50,
    p95,
    p99,
    timeouts,
    connection_failures,
    status_codes,
  }
  
}

fn percentile(sorted: &[Duration], p:f64) -> Duration {

  if sorted.is_empty() {
    return Duration::ZERO;
  } 
  
  let index: f64 = (p / 100.0) * (sorted.len()  - 1) as f64;

  sorted[index.round() as usize]

}