use crate::models::{SatelliteConfig, TelemetryPacket};
use chrono::Utc;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::time::Duration;
use tracing::{error, info, instrument};

/// Represents a single satellite simulation instance.
/// Responsible for updating its state and pushing telemetry to a sink.
pub struct SatelliteSimulator {
    config: SatelliteConfig,
    current_lat: f64,
    current_lon: f64,
    battery: f64,
    client: reqwest::Client,
}

impl SatelliteSimulator {
    /// Creates a new simulator from the provided configuration.
    pub fn new(config: SatelliteConfig) -> Self {
        Self {
            current_lat: config.initial_lat,
            current_lon: config.initial_lon,
            config,
            battery: 100.0,
            client: reqwest::Client::new(),
        }
    }

    /// Primary execution loop for a satellite.
    /// Runs until the tokio task is cancelled.
    #[instrument(skip(self), fields(source_id = %self.config.source_id))]
    pub async fn run(mut self, endpoint: String) {
        let mut interval = tokio::time::interval(Duration::from_secs_f64(1.0 / self.config.frequency));
        let mut rng = SmallRng::from_entropy();

        info!("Starting simulation thread");

        loop {
            interval.tick().await;

            // 1. Update orbital position and state
            self.tick_state();
            
            // 2. Prepare telemetry payload
            let packet = TelemetryPacket {
                id: uuid::Uuid::new_v4(),
                source_id: self.config.source_id.clone(),
                timestamp: Utc::now(),
                instrument_id: self.config.instrument_id.clone(),
                readings: serde_json::json!({
                    "lat": self.current_lat,
                    "lon": self.current_lon,
                    "alt": 350000.0 + rng.gen_range(-50.0..50.0),
                    "velocity": 7600.0 + rng.gen_range(-1.0..1.0),
                    "battery": self.battery,
                }),
            };

            // 3. Dispatch to sink (sink might be down, so we log errors but don't exit)
            if let Err(e) = self.dispatch_telemetry(&endpoint, &packet).await {
                error!("Telemetry ingestion failed: {}", e);
            }
        }
    }

    /// Internal logic to advance simulator state by one tick.
    fn tick_state(&mut self) {
        // Update orbital position (wrap-around logic for lat/lon)
        self.current_lat = wrap_lat(self.current_lat + self.config.drift_lat);
        self.current_lon = wrap_lon(self.current_lon + self.config.drift_lon);
        
        // Simple battery simulation: drain until 20%, then charge until 100% (mock cycle)
        if self.battery > 20.0 {
            self.battery -= 0.05;
        } else {
            self.battery = 100.0; // simulate a solar recharge reset
        }
    }

    /// Dispatches a telemetry packet via HTTP POST to the configured endpoint.
    async fn dispatch_telemetry(&self, endpoint: &str, packet: &TelemetryPacket) -> Result<(), reqwest::Error> {
        let resp = self.client.post(endpoint).json(packet).send().await?;
        if !resp.status().is_success() {
            error!("Server error during ingestion: {}", resp.status());
        }
        Ok(())
    }
}

/// Helper to wrap latitude between -90 and 90.
fn wrap_lat(lat: f64) -> f64 {
    if lat > 90.0 { 90.0 - (lat - 90.0) }
    else if lat < -90.0 { -90.0 + (-90.0 - lat) }
    else { lat }
}

/// Helper to wrap longitude between -180 and 180.
fn wrap_lon(lon: f64) -> f64 {
    let mut l = (lon + 180.0) % 360.0;
    if l < 0.0 { l += 360.0; }
    l - 180.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_config() -> SatelliteConfig {
        SatelliteConfig {
            source_id: "TEST-01".to_string(),
            instrument_id: "GPS-01".to_string(),
            frequency: 1.0,
            initial_lat: 0.0,
            initial_lon: 0.0,
            drift_lat: 1.0,
            drift_lon: 1.0,
        }
    }

    #[test]
    fn test_tick_state_updates_position() {
        let mut sim = SatelliteSimulator::new(mock_config());
        sim.tick_state();
        assert_eq!(sim.current_lat, 1.0);
        assert_eq!(sim.current_lon, 1.0);
    }

    #[test]
    fn test_wrap_lat() {
        assert_eq!(wrap_lat(95.0), 85.0);
        assert_eq!(wrap_lat(-95.0), -85.0);
        assert_eq!(wrap_lat(45.0), 45.0);
    }

    #[test]
    fn test_wrap_lon() {
        assert_eq!(wrap_lon(190.0), -170.0);
        assert_eq!(wrap_lon(-190.0), 170.0);
        assert_eq!(wrap_lon(90.0), 90.0);
    }

    #[test]
    fn test_battery_drain() {
        let mut sim = SatelliteSimulator::new(mock_config());
        sim.battery = 50.0;
        sim.tick_state();
        assert!(sim.battery < 50.0);
    }
}
