// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `icao-shared-kernel` pure domain.
//!
//! This crate holds the trust-critical core of the kernel — the value objects
//! and aggregate roots (`Aircraft`, `Pilot`, `Flight`, and the
//! `FlightSimulationTrainingDevice` / `FstdSession` pair), with their
//! validation invariants and derived values. It deliberately does **not** include the
//! repository protocols (async I/O) or any infrastructure adapters; those stay
//! host-side per consumer.
//!
//! ## Design
//!
//! Value objects follow *parse, don't validate*: each has a fallible smart
//! constructor returning [`Result`]`<Self, `[`ValidationError`]`>`, and once built
//! is guaranteed valid. Aggregates therefore hold already-typed fields and
//! cannot represent an invalid state.

pub mod aircraft;
pub mod error;
pub mod flight;
pub mod fstd;
pub mod fstd_session;
mod iso_date;
pub mod pilot;

pub use aircraft::{Aircraft, AircraftRegistration, AircraftType};
pub use error::ValidationError;
pub use flight::coordinate::Coordinate;
pub use flight::flight_duration::FlightDuration;
pub use flight::flight_rules::FlightRules;
pub use flight::significant_point::SignificantPoint;
pub use flight::waypoint::{validate_waypoint_code, Waypoint};
pub use flight::Flight;
pub use fstd::{DeviceDesignation, DeviceQualification, FlightSimulationTrainingDevice, FstdKind};
pub use fstd_session::FstdSession;
pub use pilot::{Licence, Pilot};

// Compiles and runs the README's examples with the doc tests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
