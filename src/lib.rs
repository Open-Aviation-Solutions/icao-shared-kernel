//! Rust port of the `icao-shared-kernel` pure domain.
//!
//! This crate ports the trust-critical, genuinely portable part of the Python
//! kernel — the value objects and aggregate roots, with their validation
//! invariants and derived values. It deliberately does **not** include the
//! repository protocols (async I/O) or any infrastructure adapters; those stay
//! host-side per consumer, exactly as in the Python package.
//!
//! See `aviation-core/tasks/0014-rust-wasm-portable-domain-core.md` for the
//! rationale: Rust is the agreed eventual single source of truth for kernel
//! validation. This is the native-Rust spike (no WASM, no language bindings
//! yet); its job is to prove validation parity against the existing pytest
//! suite.
//!
//! ## Design
//!
//! Value objects follow *parse, don't validate*: each has a fallible smart
//! constructor returning [`Result`]`<Self, `[`DomainError`]`>`, and once built
//! is guaranteed valid. Aggregates therefore hold already-typed fields and
//! cannot represent an invalid state.

pub mod aircraft;
pub mod error;
pub mod flight;
pub mod pilot;

pub use aircraft::{Aircraft, AircraftRegistration, AircraftType};
pub use error::DomainError;
pub use flight::flight_time::FlightTime;
pub use flight::waypoint::{validate_waypoint_code, Waypoint};
pub use flight::Flight;
pub use pilot::{Licence, Pilot};
