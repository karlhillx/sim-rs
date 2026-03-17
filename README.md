# Sim (Mission Simulator)

A high-performance satellite-orbit and telemetry simulator in Rust, designed for mission-critical telemetry load testing and local development.

## 🚀 Improvements (V2)

- **Multi-Source Simulation**: Support for multiple satellites running concurrently using `tokio` tasks.
- **YAML Configuration**: Define satellite profiles (frequency, initial position, drift) in `config.yaml`.
- **Simplified Orbital Drift**: Moves satellites across latitude/longitude over time instead of generating random noise.
- **Improved Logging**: Detailed, per-satellite tracing to monitor ingestion health.

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

## Options

| Option | Shorthand | Default | Description |
|--------|-----------|---------|-------------|
| `--endpoint` | `-e` | `http://127.0.0.1:3030/telemetry` | Target ingestion endpoint. |
| `--config` | `-c` | `config.yaml` | Path to satellite configuration file. |

## Future Roadmap

- [ ] **Orbital Physics**: Integrate simplified orbital mechanics for realistic lat/lon/alt drift.
- [ ] **Protobuf Ingestion**: Support for high-efficiency binary telemetry formats.
- [ ] **Web Dashboard**: A real-time local dashboard to view simulated satellite positions.

---

*Build by Mara for Karl Hill.*
