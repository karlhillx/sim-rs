use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use tokio::signal;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use sim_rs::error::SimError;
use sim_rs::models::SatelliteConfig;
use sim_rs::satellite::SatelliteSimulator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ingestion endpoint for telemetry data
    #[arg(short, long, default_value = "http://127.0.0.1:3030/telemetry")]
    endpoint: String,

    /// Path to YAML configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,

    /// Dry run: print telemetry to console instead of POSTing
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Duration in seconds to run the simulation (optional)
    #[arg(short, long)]
    duration: Option<u64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 1. Initialize modern structured logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("🚀 sim-rs: Initializing high-speed mission simulator...");

    // 2. Load and parse satellite configurations
    let config_file = File::open(&args.config)
        .map_err(SimError::ConfigReadError)?;
    let satellite_configs: Vec<SatelliteConfig> = serde_yaml::from_reader(config_file)
        .map_err(SimError::ConfigParseError)?;

    info!("📡 Loaded {} satellite profiles from {}", satellite_configs.len(), args.config.display());

    // 3. Spawn simulation tasks
    let mut tasks = Vec::new();
    for config in satellite_configs {
        let endpoint = args.endpoint.clone();
        let sim = SatelliteSimulator::new(config, args.dry_run);
        
        let task = tokio::spawn(async move {
            sim.run(endpoint).await;
        });
        tasks.push(task);
    }

    info!("⚡ Parallelism: {} tasks spawned efficiently.", tasks.len());

    // 4. Graceful shutdown handler
    if let Some(duration) = args.duration {
        info!("⏳ Simulation scheduled to run for {} seconds...", duration);
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(duration)) => {
                info!("🏁 Duration reached. Shutting down...");
            }
            _ = signal::ctrl_c() => {
                info!("🛑 Manual interrupt received. Shutting down...");
            }
        }
    } else {
        info!("🛰️ Simulation active. Press Ctrl+C to stop.");
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("🛑 Shutdown signal received. Terminating all simulation tasks...");
            }
            Err(err) => {
                error!("❌ Unable to listen for shutdown signal: {}", err);
            }
        }
    }

    Ok(())
}
