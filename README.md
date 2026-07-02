# icao-shared-kernel-rs

The **pure domain** of the `icao-shared-kernel`: validated value objects and
aggregate roots, with their invariants and derived values.

This crate is the active design surface for the kernel and the intended single
source of truth for domain validation, per `aviation-core` task
`0014-rust-wasm-portable-domain-core.md`. The older Python `icao-shared-kernel`
package is unpublished and no longer kept in lockstep — the domain is evolved
here. No WASM and no language bindings yet; those are later steps in task 0014.

## What is (and isn't) here

Domain types:

- Value objects: `Waypoint`, `Coordinate`, `SignificantPoint` (a route/flight
  path point — a coded designator *or* a lat/long coordinate), `FlightDuration`,
  `AircraftType`, `AircraftRegistration`, `Licence`.
- Aggregate roots: `Flight`, `Aircraft`, `Pilot`.
- The public `validate_waypoint_code` function.

Deliberately **out of scope** (unchanged from task 0014):

- Repository protocols (async I/O) — stay host-side per consumer.
- All infrastructure adapters (Postgres, filesystem, admin).
- WASM / WIT / Component Model, and PyO3 or other language bindings.

## Design

Value objects follow *parse, don't validate*: each has a fallible smart
constructor (`Waypoint::parse`, `Licence::new`, …) returning
`Result<Self, DomainError>`, and once constructed is guaranteed valid. The
aggregates therefore hold already-typed fields and cannot represent an invalid
state. `DomainError` is a single typed enum, one variant per validation rule.

`Flight` is a thin hub: which aircraft flew, the `departure` and `arrival`
significant points, and the optional `first_movement` / `last_movement` UTC
timestamps (movement under own power). Block time is a derived `duration()`,
computed from those timestamps when both are present rather than stored.

Each validation rule is justified against a regulatory source (ICAO Annex,
CASA Part 61) and covered by a test under `tests/`.

## Development

Requires a Rust toolchain and a C linker (`build-essential` on Debian/Ubuntu).

```sh
make help       # list targets
make dev        # build
make test       # run the test suite
make check-all  # clippy + fmt check + tests
```
