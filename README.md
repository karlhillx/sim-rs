# Sim (Telemetry Simulator)

A high-performance satellite-orbit and telemetry simulator in Rust, designed for mission-critical telemetry load testing and local development.

## Why Rust?

Following the **Drift (Telemetry Sink)** design, **Sim-rs** is a lightweight, zero-cost abstraction for simulating high-frequency telemetry streams. Perfect for testing your aerospace mission-simulation pipelines without the overhead of heavy-duty simulators.

## Core Pillars

- **Zero Overhead**: Minimal memory footprint, allowing thousands of simulated sources on a single machine.
- **Async First**: Built on `tokio` and `reqwest` for non-blocking, reliable packet transmission.
- **Configurable**: Easily adjust frequency, target endpoints, and source identities.

## Getting Started

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (v1.60+)

### Running Locally

```bash
cargo run -- --endpoint http://127.0.0.1:3030/telemetry --frequency 2.0
```

### Options

| Option | Shorthand | Default | Description |
|--------|-----------|---------|-------------|
| `--endpoint` | `-e` | `http://127.0.0.1:3030/telemetry` | Target ingestion endpoint. |
| `--frequency` | `-f` | `1.0` | Packets per second (Hz). |
| `--source_id` | `-s` | `SAT-01` | Simulated source identifier. |

## Future Roadmap

- [ ] **Orbital Physics**: Integrate simplified orbital mechanics for realistic lat/lon/alt drift.
- [ ] **Protobuf Ingestion**: Support for high-efficiency binary telemetry formats.
- [ ] **Multi-Instrument Simulation**: Define instrument specific profiles (radar, optical, sensors).

---

*Build by Mara for Karl Hill.*
