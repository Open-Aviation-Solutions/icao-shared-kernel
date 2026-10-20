# icao-shared-kernel

A Rust library of the aviation concepts that ICAO defines the same way for
every State: an aircraft's type designator and registration marks, a pilot's
licence, a flight between two significant points and the flight rules it was
flown under, and the flight simulation training devices a session can be
flown on. Each is a type that cannot hold an invalid value, so code built on
it does not have to check them again.

It is for developers of aviation software, such as logbooks and training
records, who want those concepts validated in one place. It is written first
for the [Open Aviation Solutions](https://github.com/Open-Aviation-Solutions)
tools, and is the part of them that has no country's rules in it.

## Status

Early, and not yet released: the version is `0.1.0`, nothing has been
published to crates.io, and it is used by git (see below). Until `1.0`, the
API may change between minor versions, and on `main` a breaking change lands
whenever it gives a better API, together with the changes its users need.

What is covered:

- Value objects: `Waypoint`, `Coordinate`, `SignificantPoint` (a coded
  designator *or* a latitude and longitude), `FlightDuration`, `FlightRules`,
  `AircraftType`, `AircraftRegistration`, `Licence`, `DeviceDesignation`,
  `DeviceQualification`, and the public `validate_waypoint_code` function.
- Aggregates: `Flight`, `Aircraft`, `Pilot`, `FlightSimulationTrainingDevice`
  and `FstdSession`.
- One error type, `ValidationError`, with a variant for each rule.

What is not:

- Storage: no repositories, databases or file formats. Those belong to the
  application using the crate.
- Bindings for other languages, or a WebAssembly interface.
- Any one country's rules (see "What it is not").

Known limits:

- Values are checked for their form, not against the lists they come from. A
  type designator is checked for the shape of ICAO Doc 8643's designators, not
  looked up in it; an issuing State is two capital letters, not checked
  against ISO 3166-1; a waypoint's designator is not checked against any
  State's published list.
- The types serialise with serde, but that JSON is not a stored format that is
  promised: it may change between minor versions, as the API may.

Nothing else is known not to work. The open
[issues](https://github.com/Open-Aviation-Solutions/icao-shared-kernel/issues)
are the work still to do.

## What it is not

It is not the regulations. Where it follows an ICAO Annex or Document, it
follows its formats and definitions in order to validate data, and it does not
reproduce, replace or interpret them. Whether a flight, a licence or a device
meets a State's requirements is not decided here, and nothing in it changes a
pilot's own responsibility under their State's regulations. A record that this
crate accepts is well formed, not necessarily correct.

## Using it

Until `0.1.0` is released to crates.io, depend on it by git. Pin a commit
with `rev`, since `main` can change its API at any time:

```toml
[dependencies]
icao-shared-kernel = { git = "https://github.com/Open-Aviation-Solutions/icao-shared-kernel", rev = "<commit>" }
```

Once it is released, it will be `icao-shared-kernel = "0.1"` from crates.io.

```rust
use icao_shared_kernel::{Aircraft, Flight, ValidationError};

fn main() -> Result<(), ValidationError> {
    let aircraft = Aircraft::create("C172", "VH", "ABC")?;
    let flight = Flight::create(aircraft.id, "YSBK", "YSCN")?;
    assert_eq!(flight.route_summary(), "YSBK-YSCN");

    // A designator that is not one is refused, not stored.
    assert!(Aircraft::create("cessna", "VH", "ABC").is_err());
    Ok(())
}
```

Each value object has a constructor that returns
`Result<Self, ValidationError>` (`Waypoint::parse`, `Licence::new`, …); there
is no other way to build one. The aggregates hold only validated values, so
they cannot represent an invalid state either. Each rule's rustdoc names its
regulatory source (an ICAO Annex or Document, or CASA Part 61), and each rule
has a test under `tests/`.

## Other repositories

- **pilot-logbook**, a pilot's logbook, is built on this crate.
- **icao-shared-kernel-py** is its Python bindings.
- **[au-casa](https://github.com/Open-Aviation-Solutions/au-casa)** holds
  Australia's rules (CASA Part 61), which are kept out of this crate.

This crate began as a port of an earlier Python package, which it has
replaced.

## Reporting a problem, and contributing

Report a problem, or propose a change, in this repository's
[issues](https://github.com/Open-Aviation-Solutions/icao-shared-kernel/issues).
The organisation's contributing guide, security policy and code of conduct
are not written yet; they are tracked in Open-Aviation-Solutions/pilot-logbook#108.

## Development

Requires a Rust toolchain, a C linker (`build-essential` on Debian/Ubuntu),
and Go, which `make check` uses to install
[addlicense](https://github.com/google/addlicense) when it is not on the
`PATH`.

```sh
make help            # list targets
make dev             # build
make test            # run the test suite
make check           # clippy, format check, licence notices, tests
```

## Licence

[Mozilla Public License 2.0](LICENSE). Each source file carries the licence's
notice.
