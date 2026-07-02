//! The `icao-shared-kernel` pure domain.
//!
//! This crate holds the trust-critical core of the kernel — the value objects
//! and aggregate roots (`Aircraft`, `Pilot`, `Flight`), with their validation
//! invariants and derived values. It deliberately does **not** include the
//! repository protocols (async I/O) or any infrastructure adapters; those stay
//! host-side per consumer.
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
