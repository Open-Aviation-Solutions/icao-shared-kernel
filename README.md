# icao-shared-kernel-rs

A native Rust port of the **pure domain** of the
[`icao-shared-kernel`](../aviation-core) Python package: validated value
objects and aggregate roots, with their invariants and derived values.

This crate exists to execute the higher-value, lower-risk half of
`aviation-core` task `0014-rust-wasm-portable-domain-core.md`: making Rust the
**single source of truth** for kernel validation. It is the *native-Rust spike*
— **no WASM, no language bindings yet**. Its job is to prove validation parity
against the existing Python pytest suite before any binding or distribution
decision is made.

## What is (and isn't) here

Ported (the trust-critical, portable part):

- Value objects: `Waypoint`, `FlightTime`, `AircraftType`,
  `AircraftRegistration`, `Licence`.
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
state. `DomainError` is a single typed enum — the Rust analogue of the typed
errors Pydantic raises.

The waypoint error messages match the Python `validate_waypoint_code` strings
exactly (that validator is our own code, asserted on in the pytest suite). The
other rules are matched *behaviourally* — they reject the same inputs — rather
than reproducing Pydantic's framework message strings.

## Development

Requires a Rust toolchain and a C linker (`build-essential` on Debian/Ubuntu).

```sh
make help       # list targets
make dev        # build
make test       # run the parity test suite
make check-all  # clippy + fmt check + tests
```

## Parity contract

The Python pytest suite under `aviation-core/tests/models/` is the reference
spec. Every behaviour it asserts has an equivalent test under `tests/` here.
One faithful subtlety worth noting: a waypoint with no letters (e.g. `12345`)
is rejected, because the Python validator uses `str.isupper()`, which is `False`
for an all-digit string.
