use std::time::Duration;

use async_rate_limiter::RateLimiterServer;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: u16,

    #[arg(short, long)]
    limit: usize,

    #[arg(short, long)]
    duration_ms: u64,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let rate_limiter = RateLimiterServer::new(
        args.limit,
        Duration::from_millis(
            args.duration_ms
        ),
    );
    rate_limiter
        .serve(args.port)
        .await
        .unwrap();
}
