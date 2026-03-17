use chrono::Utc;
use clap::Parser;
use rand::Rng;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://127.0.0.1:3030/telemetry")]
    endpoint: String,

    #[arg(short, long, default_value_t = 1.0)]
    frequency: f64,

    #[arg(short, long, default_value = "SAT-01")]
    source_id: String,
}

#[derive(Serialize)]
struct TelemetryPacket {
    source_id: String,
    timestamp: String,
    instrument_id: String,
    readings: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting sim-rs for source: {}", args.source_id);
    info!("Targeting endpoint: {}", args.endpoint);

    let client = reqwest::Client::new();
    let mut rng = rand::thread_rng();

    loop {
        let packet = TelemetryPacket {
            source_id: args.source_id.clone(),
            timestamp: Utc::now().to_rfc3339(),
            instrument_id: "GPS-01".to_string(),
            readings: serde_json::json!({
                "lat": rng.gen_range(-90.0..90.0),
                "lon": rng.gen_range(-180.0..180.0),
                "alt": rng.gen_range(200000.0..400000.0),
                "velocity": rng.gen_range(7000.0..8000.0),
                "battery": rng.gen_range(80.0..100.0),
            }),
        };

        match client.post(&args.endpoint).json(&packet).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    info!("Successfully sent telemetry packet");
                } else {
                    error!("Server returned error status: {}", resp.status());
                }
            }
            Err(e) => error!("Failed to send packet: {}", e),
        }

        sleep(Duration::from_secs_f64(1.0 / args.frequency)).await;
    }
}
