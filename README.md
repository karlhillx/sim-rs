# sim-rs: High-Performance Satellite Mission Simulator

`sim-rs` is a high-speed, scale-testing engine designed for simulating satellite orbital dynamics and telemetry streams. It is the companion simulator to `drift-rs`, used for validating telemetry ingestion pipelines, dashboard resilience, and mission control performance under load.

## Core Capabilities

- **Physics-Based Drift:** Realistic orbital simulation including altitude decay, velocity changes, and wrap-around spherical coordinates.
- **Massive Parallelism:** Built on `tokio`, capable of simulating thousands of independent satellite nodes on a single machine.
- **Resilient Telemetry:** Integrated retry mechanisms with exponential backoff to handle intermittent network failures.
- **High-Speed Execution:** Written in Rust for maximum efficiency and low resource footprint.

## Quick Start

### Simulation Modes

**1. Scaled Load Testing**
Simulate real-world conditions by POSTing telemetry to a target endpoint:
```bash
cargo run -- --endpoint http://your-api.com/telemetry
```

**2. Dry Run / Console Debugging**
Validate your configuration and state drift without network overhead:
```bash
cargo run -- --dry-run
```

**3. Scheduled Mission Simulation**
Run the simulation for a specific duration:
```bash
cargo run -- --duration 300 --dry-run
```

## Configuration

Satellite profiles are managed in `config.yaml`. Example:

```yaml
- source_id: ISS-SIM
  instrument_id: ENV-SENSE-1
  frequency: 1.0        # Hz
  initial_lat: 51.6
  initial_lon: 0.0
  initial_alt: 408000.0 # Meters
  initial_velocity: 7660.0 # m/s
  drift_lat: 0.05
  drift_lon: 0.1
  drift_alt: -0.1
  drift_velocity: 0.01
```

## Resilience

The simulator uses `reqwest-retry` to ensure that telemetry packets are delivered even during temporary network outages. It implements an exponential backoff strategy (up to 3 retries) for all HTTP POST operations.
