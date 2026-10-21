# icao-shared-kernel — Working Instructions

Value objects and aggregates for International Civil Aviation Organization
concepts, in Rust. It began as a port of an earlier Python package, which it
has replaced.

## Purpose and scope

This crate is the **active design surface** for the aviation concepts ICAO
standardises across States (aircraft, pilots' licences, flights, flight
simulation training devices), and the single source of truth for their
validation.

In scope: value objects, aggregate roots, their invariants and derived values.

Out of scope (do not add here): repository protocols / async I/O, infrastructure
adapters (Postgres, filesystem, admin), and — for now — WASM/WIT and any
language bindings (PyO3, etc.). Bindings live in their own repositories.

## Validation rules are the contract

Every value object owns its invariants and is the single place they live. Each
rule should be justified against an ICAO source (an Annex or Document). A State's
own rules, such as Australia's CASA Part 61, stay out of this crate.

Each rule is covered by a test in `tests/`. When you change a rule, change its
test alongside it and record the regulatory basis in the doc comment.

## Conventions

- **Discuss before implementing** non-trivial design changes.
- **Regulatory terms only**: names and terminology must be verifiable against
  the ICAO Annexes and the procedures they rest on (Doc 4444, PANS-ATM, for a
  flight plan's items).
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

`pilot-logbook` and `icao-shared-kernel-py` (the Python bindings) depend on
this crate. `au-casa` does not yet, but its simulator classification (its task
`0002`) will build on the FSTD aggregates.

Choose the best API, not the compatible one: while the crate is on `0.x`, a
breaking change is fine, landed with a matching PR in each consumer. Releases
follow semver for `0.x`: a release with a breaking change bumps the minor
version.

## Releasing

Bump `version` in `Cargo.toml` in a pull request, and merge it: that is the
release. On each push to `main`, the Release workflow
(`.github/workflows/release.yml`) asks crates.io whether `Cargo.toml`'s version
is there yet. When it is not, it runs `make check`, publishes it through trusted
publishing (no crates.io token is kept in the repository), and tags the commit
`v<version>`. Don't push version tags by hand: nothing runs on them.

Only `main` can release. The publishing job runs in the `release` environment,
which GitHub allows only on `main`, and crates.io's trusted publisher accepts
only that environment; `main` changes only through a pull request with CI
passing.

## Commands

```sh
make test       # cargo test — the test suite
make lint       # cargo clippy -- -D warnings
make fmt        # cargo fmt
make check      # lint + fmt check + licence headers + test
```
