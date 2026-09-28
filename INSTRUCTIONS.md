# icao-shared-kernel-rs — Working Instructions

A native Rust port of the pure domain of the `icao-shared-kernel` Python
package (sibling repo `../aviation-core`). This crate is the native-Rust spike
from `aviation-core` task `0014-rust-wasm-portable-domain-core.md`.

## Purpose and scope

This crate is the **active design surface** for the kernel's domain — the
direction agreed in task 0014, with Rust as the intended single source of truth
for domain validation. The Python `icao-shared-kernel` package is unpublished
and is **not** kept in lockstep; the domain is now being evolved here, and the
two have deliberately diverged (e.g. the `Licence` identity, `SignificantPoint`
route endpoints, and movement-timestamp modelling below have no Python
equivalent yet). If the Python package is ever revived, it follows this crate,
not the other way round.

In scope: value objects, aggregate roots, their invariants and derived values.

Out of scope (do not add here): repository protocols / async I/O, infrastructure
adapters (Postgres, filesystem, admin), and — for now — WASM/WIT and any
language bindings (PyO3, etc.). Those are separate, later steps in task 0014.

## Validation rules are the contract

Every value object owns its invariants and is the single place they live. Each
rule should be justified against a regulatory source (ICAO Annex, CASA Part 61)
rather than copied from the earlier Python model — where the two differ, this
crate is correct. The Python pytest suite under `aviation-core/tests/models/`
is now only *historical reference* for how a rule once behaved, not a spec to
match.

Each rule is covered by a test in `tests/`. When you change a rule, change its
test alongside it and record the regulatory basis in the doc comment.

## Conventions

- **Discuss before implementing** non-trivial design changes, consistent with
  the `aviation-core` working style.
- **Regulatory terms only**: names and terminology must be verifiable against
  the same regulatory sources as the Python kernel (ICAO Annexes, CASA Part 61),
  and the ICAO procedures they rest on (Doc 4444, PANS-ATM, for a flight plan's
  items).
- **No vendored ICAO-copyrighted data** — only validation *rules* (facts) are
  ported, consistent with `aviation-core` task 0012.
- Value objects use *parse, don't validate*: a fallible smart constructor
  returning `Result<Self, ValidationError>`; no public way to build an invalid value.
- Module layout: each aggregate is a module file (`flight.rs`) with a sibling
  directory of the same name holding its value-object submodules
  (`flight/waypoint.rs`, `flight/coordinate.rs`, `flight/significant_point.rs`,
  `flight/flight_duration.rs`, `flight/flight_rules.rs`; `aircraft/aircraft_type.rs`,
  `aircraft/registration.rs`; `pilot/licence.rs`; `fstd/designation.rs`,
  `fstd/qualification.rs`). An aggregate with no value objects of its own has
  no sibling directory (`fstd_session.rs`). `lib.rs` re-exports the value
  objects so the public API stays flat.

## Consumers

`pilot-logbook` is the only consumer, and depends on this crate's `main` branch
(its lockfile pins a commit). Choose the best API, not the compatible one: a
breaking change is fine, landed with a matching `pilot-logbook` PR that updates
its lockfile to it.

## Commands

```sh
make test       # cargo test — the parity suite
make lint       # cargo clippy -- -D warnings
make fmt        # cargo fmt
make check      # lint + fmt check + test
```
