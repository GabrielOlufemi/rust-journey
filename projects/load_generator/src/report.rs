use std::time::Duration;
use crate::stats::Summary;

pub fn print_report(summary: &Summary) {
  println!("Load test results");
  println!("{}", "-".repeat(30));

  println!(
    "{:<12}{:>6}  ({} ok, {} failed)",
    "Requests", summary.total, summary.successes, summary.failures
  );

  println!("{:<12}{:>6.2} s", "Wall clock", summary.wall_clock.as_secs_f64());
  println!("{:<12}{:>6.1} req/s", "Throughput", summary.throughput);

  println!();
  println!("Latency");
  println!("  {:<10}{:>9.1} ms", "min", ms(summary.min));
  println!("  {:<10}{:>9.1} ms", "max", ms(summary.max));
  println!("  {:<10}{:>9.1} ms", "mean", ms(summary.mean));
  println!("  {:<10}{:>9.1} ms", "p50", ms(summary.p50));
  println!("  {:<10}{:>9.1} ms", "p95", ms(summary.p95));
  println!("  {:<10}{:>9.1} ms", "p99", ms(summary.p99));

  println!();
  println!("Status codes");
  for (code, count) in &summary.status_codes {
    println!("  {:<10}{:>9}", code, count);
  }

  println!();
  println!("Errors");
  println!("  {:<22}{:>5}", "timeouts", summary.timeouts);
  println!("  {:<22}{:>5}", "connection failures", summary.connection_failures);
}

fn ms(d: Duration) -> f64 {
  d.as_secs_f64() * 1000.0
}
