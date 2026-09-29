# Simulator device and session aggregates

**Status:** done, archived 2026-09-29 (commit `2fb6f41`).

## Purpose

The kernel cannot currently represent a training session flown on a
simulator. `Flight.aircraft_id: Uuid` is mandatory and `Aircraft` requires
both a Doc 8643 type designator and Annex 7 registration marks, so a session
flown on a device that is not an aircraft has nowhere to go.

The old Python kernel worked around this by modelling a simulator *as* an
`Aircraft` carrying `sim_type` / `is_regulatory_sim` fields. Task `0011` in
`icao-shared-kernel` removed those fields, and the tightened value objects in
this crate mean the workaround no longer even constructs: `AircraftType`
rejects anything outside the Doc 8643 shape (`ZZZZ` means *"real aircraft, no
assigned designator"*, not *"not an aircraft"*), and `AircraftRegistration`
caps the registration mark at 5 characters, rejecting the `"UNKNWN"`
placeholder the existing importer uses.

`pilot-logbook`'s currency and totals metrics are blocked on this — six files
still read the removed fields.

## Decisions

**Sibling aggregates, not a widening of `Flight`.** A simulator session is not
a physical flight event, and the kernel's thin-hub discipline forbids widening
`Flight` for a field group. Two new aggregates sit alongside it.

**The device is ICAO-universal, its recognition is national.** The kernel owns
device identity, apparatus kind, and the *current* qualification. Whether a
given qualification counts toward a licensing requirement is a national
question and stays out of this crate — see `au-casa` task `0002`, which holds
the full regulatory verification behind these decisions and is not repeated
here.

**`FlightSimulationTrainingDevice`** — the physical device.

- `kind`: `FlightSimulator` / `FlightProceduresTrainer` /
  `BasicInstrumentFlightTrainer`.
- `designation`: free-text identification of the device (manufacturer, model,
  serial), non-empty.
- `qualification: Option<DeviceQualification>` — `None` for an unqualified
  device, which is a first-class case, not an error. A personal device is
  logged normally and simply earns no credit.

**`DeviceQualification`** (value object) — issuing State and authority, level,
and a validity period, with `is_in_force_on(date)`. Qualifications are
time-bounded, so "is this device qualified" is unanswerable without a date.

`level` is a validated non-empty `String`, deliberately **not** an enum
(YAGNI): nothing branches on it yet, and the eventual enum would span four
schemes (ICAO Doc 9625 Types I–VII, simulator Levels A–D, FAA FTD Levels 4–7,
EASA FTD Levels 1–3). Build it when a consumer needs to compare levels.

**`FstdSession`** — the session, sibling to `Flight`.

- `simulated_aircraft_type: Option<AircraftType>` lives **on the session, not
  the device**. A qualified device is bound to one simulated aircraft by its
  qualification certificate, but an unqualified personal device is routinely
  flown as a different type each session. Session-level placement serves both
  with one field; `None` covers a device not representing any specific type.
- `departure` / `arrival` are `Option<SignificantPoint>` — a session may be
  pure manoeuvre practice with no route.
- `start` / `end` mirror `Flight`'s movement timestamps, with the same derived
  `duration()`.

### Naming — from ICAO Annex 1, verified

ICAO Annex 1 (Personnel Licensing), Fourteenth Edition, July 2022, Chapter 1:

> **Flight simulation training device (FSTD).** Any one of the following three
> types of apparatus in which flight conditions are simulated on the ground:
>
> A **flight simulator**, which provides an accurate representation of the
> flight deck of a particular aircraft type […];
>
> A **flight procedures trainer**, which provides a realistic flight deck
> environment […] and the performance and flight characteristics of aircraft
> of a particular class;
>
> A **basic instrument flight trainer**, which is equipped with appropriate
> instruments, and which simulates the flight deck environment of an aircraft
> in flight […] in instrument flight conditions.

This is the kernel's term and the source of all three `kind` variants.

Two things it settles that a CASR-only reading got wrong:

- **ICAO's FSTD is generic**; approval is a separate matter for each State's
  licensing authority. CASR reg 61.010's FSTD, by contrast, means specifically
  a *recognised* device. Because the kernel follows ICAO, an unqualified
  personal device is a perfectly ordinary `FlightSimulationTrainingDevice`
  with `qualification: None` — no term is being misused.
- **The middle kind is a *flight procedures trainer***, defined by aircraft
  *class*. CASR's equivalent triple ("synthetic training device") names its
  middle kind a *flight training device* instead. The two are similar in role
  but are not the same term, so the national layer maps onto the ICAO triple
  rather than replacing it — see `au-casa` task `0002`.

The session is `FstdSession` rather than `SimulatorSession` for the same
reason: only one of the three kinds is a simulator.

## Acceptance criteria

- [x] `FlightSimulationTrainingDevice` aggregate with `new` / `with` / `create`
      constructors matching the `Aircraft` pattern.
- [x] `FstdKind` — the three Annex 1 variants, each doc comment paraphrasing
      its Annex 1 description.
- [x] `DeviceDesignation` value object — non-empty, *parse don't validate*.
- [x] `DeviceQualification` value object — issuing State (ISO 3166-1 alpha-2)
      and authority, level, validity period; rejects a period whose end
      precedes its start; `is_in_force_on` inclusive at both ends.
- [x] `FstdSession` aggregate with optional simulated type, optional
      route, optional timestamps, and a `duration()` derived like `Flight`'s.
- [x] `lib.rs` re-exports the new types so the public API stays flat.
- [x] Tests covering each invariant and each derived value.
- [x] `make check` passes (clippy with `-D warnings`, fmt, tests).

## Cross-repo notes

- `au-casa` task `0002` — the national layer (`FstdRecognition`,
  `counts_for_part61`), and the regulatory verification for all of the above.
  Blocked on this task.
- `pilot-logbook` — needs its own task: snapshot the recognition onto the
  logbook record at logging time, give `EnrichedEntry` a discriminated
  flown-aircraft vs simulated-session subject, and rewire the six call sites
  that still import from the renamed `aviation_core` package.

## Notes for the implementer

`validate_issuing_state` was private to `pilot/licence.rs`; it moved to
`error.rs` as `check_issuing_state` so `DeviceQualification` can reuse it.
Behaviour is unchanged.
