# icao-shared-kernel-rs — Working Instructions

A native Rust port of the pure domain of the `icao-shared-kernel` Python
package (sibling repo `../aviation-core`). This crate is the native-Rust spike
from `aviation-core` task `0014-rust-wasm-portable-domain-core.md`.

## Purpose and scope

This crate is intended as the **eventual single source of truth** for the
kernel's domain validation — the direction agreed in task 0014, with Pydantic
retired from that role over time. To avoid the worst outcome in a regulated
domain (two authoritative implementations that drift), the Python and Rust
sides must never *both* own validation rules. While both exist, the Python
package remains authoritative and this crate proves it can replace that role.

In scope: value objects, aggregate roots, their invariants and derived values.

Out of scope (do not add here): repository protocols / async I/O, infrastructure
adapters (Postgres, filesystem, admin), and — for now — WASM/WIT and any
language bindings (PyO3, etc.). Those are separate, later steps in task 0014.

## Parity is the contract

The Python pytest suite under `aviation-core/tests/models/` is the reference
specification. Any change to a validation rule here must keep parity with that
suite (or be made in lockstep with a deliberate change on the Python side).
When porting or changing a rule, port the *behaviour the Python code actually
has*, not an idealised version — e.g. an all-digit waypoint is rejected because
Python's `str.isupper()` is `False` for a string with no cased characters.

Match the Python error *message strings* only for our own `validate_waypoint_code`
(the pytest suite asserts on those). For rules enforced by Pydantic, match
behaviour (which inputs are accepted/rejected), not the framework's message text.

## Conventions

- **Discuss before implementing** non-trivial design changes, consistent with
  the `aviation-core` working style.
- **Regulatory terms only**: names and terminology must be verifiable against
  the same regulatory sources as the Python kernel (ICAO Annexes, CASA Part 61).
- **No vendored ICAO-copyrighted data** — only validation *rules* (facts) are
  ported, consistent with `aviation-core` task 0012.
- Value objects use *parse, don't validate*: a fallible smart constructor
  returning `Result<Self, DomainError>`; no public way to build an invalid value.
- Module layout: each aggregate is a module file (`flight.rs`) with a sibling
  directory of the same name holding its value-object submodules
  (`flight/waypoint.rs`, `flight/flight_time.rs`). `lib.rs` re-exports the
  value objects so the public API stays flat.

## Commands

```sh
make test       # cargo test — the parity suite
make lint       # cargo clippy -- -D warnings
make fmt        # cargo fmt
make check-all  # lint + fmt check + test
```
