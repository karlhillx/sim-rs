# Sim (Mission Simulator)

A high-performance satellite-orbit and telemetry simulator in Rust, designed for mission-critical telemetry load testing and local development.

## 🚀 Improvements (V3 - Modern Best Practices)

- **Crate Restructuring**: Separated concerns into a library (`lib.rs`) and a binary (`main.rs`), allowing for better testing and reuse.
- **Robust Error Handling**: Integrated `thiserror` for library-level error definitions and `anyhow` for binary-level error management.
- **Unit Testing**: Included unit tests for core logic (orbital drift, latitude/longitude wrapping, and battery cycles).
- **Graceful Shutdown**: Added a `SIGINT` (Ctrl+C) handler for clean task termination.
- **GitHub Actions**: Integrated CI workflow for automated build, test, and clippy checks.
- **Structured Telemetry**: Enforced strongly-typed telemetry packets with UUIDv4 identifiers.

## Core Pillars

- **Zero Overhead**: Minimal memory footprint, allowing thousands of simulated sources on a single machine.
- **Async First**: Built on `tokio` and `reqwest` for non-blocking, reliable packet transmission.
- **Scalable**: Concurrent execution of satellite profiles defined in a central configuration.

## Getting Started

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (v1.60+)

### Running Locally

```bash
cargo run -- --config config.yaml --endpoint http://127.0.0.1:3030/telemetry
```

### Running Tests

```bash
cargo test
```

### Configuration (config.yaml)

```yaml
- source_id: SAT-01
  instrument_id: GPS-01
  frequency: 1.0        # Packets per second (Hz)
  initial_lat: 45.0
  initial_lon: -75.0
  drift_lat: 0.05       # Degrees per second
  drift_lon: 0.1
```

## Future Roadmap

- [ ] **Orbital Physics**: Integrate simplified orbital mechanics for realistic lat/lon/alt drift.
- [ ] **Protobuf Ingestion**: Support for high-efficiency binary telemetry formats.
- [ ] **Web Dashboard**: A real-time local dashboard to view simulated satellite positions.

---

*Build by Mara for Karl Hill.*
