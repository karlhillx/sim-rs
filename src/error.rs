use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimError {
    #[error("failed to read configuration file: {0}")]
    ConfigReadError(#[from] std::io::Error),

    #[error("failed to parse configuration: {0}")]
    ConfigParseError(#[from] serde_yaml::Error),

    #[error("telemetry ingestion failed: {0}")]
    IngestionError(#[from] reqwest::Error),

    #[error("task JoinError: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("unknown simulation error")]
    Unknown,
}
