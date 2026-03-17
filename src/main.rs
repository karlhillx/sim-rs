mod satellite;

use clap::Parser;
use satellite::{SatelliteConfig, SatelliteSimulator};
use std::fs::File;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://127.0.0.1:3030/telemetry")]
    endpoint: String,

    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Initializing sim-rs mission simulator...");
    info!("Targeting ingestion endpoint: {}", args.endpoint);

    // 1. Load satellite configurations
    let config_file = File::open(&args.config)?;
    let satellite_configs: Vec<SatelliteConfig> = serde_yaml::from_reader(config_file)?;

    info!("Loaded {} satellite profiles from {}", satellite_configs.len(), args.config.display());

    // 2. Spawn simulation tasks for each satellite
    let mut handles = Vec::new();
    for config in satellite_configs {
        let endpoint = args.endpoint.clone();
        let sim = SatelliteSimulator::new(config);
        
        let handle = tokio::spawn(async move {
            sim.run(endpoint).await;
        });
        handles.push(handle);
    }

    // 3. Wait for all tasks to complete (they won't unless there's an error)
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
