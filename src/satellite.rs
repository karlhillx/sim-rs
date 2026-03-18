use crate::models::{SatelliteConfig, TelemetryPacket};
use chrono::Utc;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Duration;
use tracing::{error, info, instrument};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

/// Represents a single satellite simulation instance.
/// Responsible for updating its state and pushing telemetry to a sink.
pub struct SatelliteSimulator {
    config: SatelliteConfig,
    current_lat: f64,
    current_lon: f64,
    current_alt: f64,
    current_velocity: f64,
    battery: f64,
    client: ClientWithMiddleware,
    dry_run: bool,
}

impl SatelliteSimulator {
    /// Creates a new simulator from the provided configuration.
    pub fn new(config: SatelliteConfig, dry_run: bool) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            current_lat: config.initial_lat,
            current_lon: config.initial_lon,
            current_alt: config.initial_alt,
            current_velocity: config.initial_velocity,
            config,
            battery: 100.0,
            client,
            dry_run,
        }
    }

    /// Primary execution loop for a satellite.
    /// Runs until the tokio task is cancelled.
    #[instrument(skip(self), fields(source_id = %self.config.source_id))]
    pub async fn run(mut self, endpoint: String) {
        let mut interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.config.frequency));
        
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
                    "alt": self.current_alt,
                    "velocity": self.current_velocity,
                    "battery": self.battery,
                }),
            };

            // 3. Dispatch to sink (sink might be down, so we log errors but don't exit)
            if self.dry_run {
                info!("DRY RUN - Telemetry: {:?}", packet);
            } else if let Err(e) = self.dispatch_telemetry(&endpoint, &packet).await {
                error!("Telemetry ingestion failed: {}", e);
            }
        }
    }

    /// Internal logic to advance simulator state by one tick.
    fn tick_state(&mut self) {
        let dt = 1.0 / self.config.frequency;
        let mut rng = StdRng::from_entropy();

        // Physics-based drift with slight randomness
        self.current_lat = wrap_lat(self.current_lat + (self.config.drift_lat * dt) + rng.gen_range(-0.001..0.001));
        self.current_lon = wrap_lon(self.current_lon + (self.config.drift_lon * dt) + rng.gen_range(-0.001..0.001));
        self.current_alt += (self.config.drift_alt * dt) + rng.gen_range(-5.0..5.0);
        self.current_velocity += (self.config.drift_velocity * dt) + rng.gen_range(-0.1..0.1);
        
        // Simple battery simulation: drain until 20%, then charge until 100% (mock cycle)
        if self.battery > 20.0 {
            self.battery -= 0.05 * dt;
        } else {
            self.battery = 100.0; // simulate a solar recharge reset
        }
    }

    /// Dispatches a telemetry packet via HTTP POST to the configured endpoint.
    async fn dispatch_telemetry(&self, endpoint: &str, packet: &TelemetryPacket) -> Result<(), reqwest_middleware::Error> {
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
            initial_alt: 350000.0,
            initial_velocity: 7600.0,
            drift_lat: 1.0,
            drift_lon: 1.0,
            drift_alt: 0.0,
            drift_velocity: 0.0,
        }
    }

    #[test]
    fn test_tick_state_updates_position() {
        let mut sim = SatelliteSimulator::new(mock_config(), true);
        sim.tick_state();
        // Use approx comparisons for float math with noise
        assert!((sim.current_lat - 1.0).abs() < 0.01);
        assert!((sim.current_lon - 1.0).abs() < 0.01);
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
        let mut sim = SatelliteSimulator::new(mock_config(), true);
        sim.battery = 50.0;
        sim.tick_state();
        assert!(sim.battery < 50.0);
    }
}
