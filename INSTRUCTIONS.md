# icao-shared-kernel — Working Instructions

The pure domain of the ICAO shared kernel, in Rust. It began as a port of an
earlier Python package, which it has replaced.

## Purpose and scope

This crate is the **active design surface** for the kernel's domain, and the
single source of truth for domain validation. The earlier Python package is
retired and was never kept in lockstep: the domain is evolved here.

In scope: value objects, aggregate roots, their invariants and derived values.

Out of scope (do not add here): repository protocols / async I/O, infrastructure
adapters (Postgres, filesystem, admin), and — for now — WASM/WIT and any
language bindings (PyO3, etc.). Bindings live in their own repositories.

## Validation rules are the contract

Every value object owns its invariants and is the single place they live. Each
rule should be justified against a regulatory source (ICAO Annex, CASA Part 61)
rather than copied from the earlier Python model — where the two differ, this
crate is correct.

Each rule is covered by a test in `tests/`. When you change a rule, change its
test alongside it and record the regulatory basis in the doc comment.

## Conventions

- **Discuss before implementing** non-trivial design changes.
- **Regulatory terms only**: names and terminology must be verifiable against
  the regulatory sources (ICAO Annexes, CASA Part 61),
  and the ICAO procedures they rest on (Doc 4444, PANS-ATM, for a flight plan's
  items).
- **No vendored ICAO-copyrighted data** — only validation *rules* (facts) are
  ported.
- **The MPL notice heads each file** in `src/`, `tests/`, the `Makefile`,
  `Cargo.toml` and the CI workflow (the licence's Exhibit A, in that file's
  comment syntax); `addlicense -l mpl <file>` adds it. `make license-check`,
  part of `make check`, runs `addlicense -check` over the tracked files, which
  fails on a Rust, TOML or YAML file with no licence header, though it does not
  check that the header is the MPL's wording. It does not check the
  `Makefile`, which it does not recognise.
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

`pilot-logbook` and `icao-shared-kernel-py` (the Python bindings) are the
consumers, and each depends on this crate's `main` branch (its lockfile pins a
commit). Choose the best API, not the compatible one: a breaking change is fine,
landed with a matching PR in each consumer that updates its lockfile to it.

## Commands

```sh
make test       # cargo test — the test suite
make lint       # cargo clippy -- -D warnings
make fmt        # cargo fmt
make check      # lint + fmt check + licence headers + test
```
