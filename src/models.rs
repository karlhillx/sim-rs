use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryPacket {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub instrument_id: String,
    pub readings: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reading {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub velocity: f64,
    pub battery: f64,
}
