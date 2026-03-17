use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SatelliteConfig {
    pub source_id: String,
    pub instrument_id: String,
    pub frequency: f64, // Hz
    pub initial_lat: f64,
    pub initial_lon: f64,
    pub drift_lat: f64, // degree per second
    pub drift_lon: f64, // degree per second
}

#[derive(Serialize)]
struct TelemetryPacket {
    source_id: String,
    timestamp: String,
    instrument_id: String,
    readings: serde_json::Value,
}

pub struct SatelliteSimulator {
    config: SatelliteConfig,
    current_lat: f64,
    current_lon: f64,
    battery: f64,
}

impl SatelliteSimulator {
    pub fn new(config: SatelliteConfig) -> Self {
        Self {
            current_lat: config.initial_lat,
            current_lon: config.initial_lon,
            config,
            battery: 100.0,
        }
    }

    pub async fn run(mut self, endpoint: String) {
        let client = reqwest::Client::new();
        let mut interval = tokio::time::interval(Duration::from_secs_f64(1.0 / self.config.frequency));
        let mut rng = rand::thread_rng();

        info!("Starting simulation for satellite: {}", self.config.source_id);

        loop {
            interval.tick().await;

            // 1. Update orbital position (simplified drift)
            self.current_lat = (self.current_lat + self.config.drift_lat + 90.0) % 180.0 - 90.0;
            self.current_lon = (self.current_lon + self.config.drift_lon + 180.0) % 360.0 - 180.0;
            
            // 2. Simulate battery drain/charge cycle
            self.battery = (self.battery - 0.01 + 100.0) % 100.0;

            let packet = TelemetryPacket {
                source_id: self.config.source_id.clone(),
                timestamp: Utc::now().to_rfc3339(),
                instrument_id: self.config.instrument_id.clone(),
                readings: serde_json::json!({
                    "lat": self.current_lat,
                    "lon": self.current_lon,
                    "alt": 350000.0 + rng.gen_range(-50.0..50.0), // add slight noise
                    "velocity": 7600.0 + rng.gen_range(-1.0..1.0),
                    "battery": self.battery,
                }),
            };

            // 3. Dispatch to sink
            match client.post(&endpoint).json(&packet).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        error!("{}: Ingestion failure ({})", self.config.source_id, resp.status());
                    }
                }
                Err(e) => error!("{}: Connection error: {}", self.config.source_id, e),
            }
        }
    }
}
