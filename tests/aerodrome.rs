// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The aerodrome a flight departs from or arrives at: listed by its location
//! indicator, or unlisted and described by its name, its position or both.

use icao_shared_kernel::{Aerodrome, Coordinate, UnlistedAerodrome, ValidationError};
use rstest::rstest;
use rust_decimal_macros::dec;
use serde_json::json;

fn position() -> Coordinate {
    Coordinate::new(dec!(-33.9461), dec!(151.1772)).unwrap()
}

#[test]
fn a_listed_aerodrome_is_its_location_indicator() {
    let aerodrome = Aerodrome::listed("YSBK").unwrap();
    assert_eq!(aerodrome.to_string(), "YSBK");
    assert_eq!(serde_json::to_value(&aerodrome).unwrap(), json!("YSBK"));
    assert_eq!(
        serde_json::from_value::<Aerodrome>(json!("YSBK")).unwrap(),
        aerodrome
    );
}

#[rstest]
#[case("YSB")] // a waypoint's designator, not a location indicator
#[case("YSBK1")]
fn a_designator_that_is_not_a_location_indicator_is_refused(#[case] value: &str) {
    assert_eq!(
        Aerodrome::listed(value).unwrap_err(),
        ValidationError::LocationIndicator(value.to_owned())
    );
    let error = serde_json::from_value::<Aerodrome>(json!(value)).unwrap_err();
    assert_eq!(
        error.to_string(),
        format!("location indicator '{value}' must be four uppercase letters")
    );
}

#[test]
fn zzzz_is_refused_as_a_listed_aerodrome() {
    assert_eq!(
        Aerodrome::listed("ZZZZ").unwrap_err(),
        ValidationError::NoLocationIndicator
    );
    let error = serde_json::from_value::<Aerodrome>(json!("ZZZZ")).unwrap_err();
    assert_eq!(
        error.to_string(),
        ValidationError::NoLocationIndicator.to_string()
    );
}

#[rstest]
#[case(Some("Farm strip"), None, "Farm strip")]
#[case(None, Some(position()), "-33.9461,151.1772")]
#[case(Some("Farm strip"), Some(position()), "Farm strip (-33.9461,151.1772)")]
fn an_unlisted_aerodrome_has_a_name_or_a_position(
    #[case] name: Option<&str>,
    #[case] position: Option<Coordinate>,
    #[case] shown: &str,
) {
    let aerodrome = Aerodrome::unlisted(name.map(str::to_owned), position.clone()).unwrap();
    let Aerodrome::Unlisted(unlisted) = &aerodrome else {
        panic!("should be unlisted");
    };
    assert_eq!(unlisted.name(), name);
    assert_eq!(unlisted.position(), position.as_ref());
    assert_eq!(aerodrome.to_string(), shown);
}

#[test]
fn an_unlisted_aerodrome_with_neither_is_refused() {
    let error = UnlistedAerodrome::new(None, None).unwrap_err();
    assert_eq!(error, ValidationError::UnlistedAerodrome);
    assert_eq!(
        error.to_string(),
        "an unlisted aerodrome must have a name or a position"
    );
}

#[test]
fn a_name_is_held_without_its_surrounding_whitespace() {
    let unlisted = UnlistedAerodrome::new(Some("  Farm strip\t".to_owned()), None).unwrap();
    assert_eq!(unlisted.name(), Some("Farm strip"));
    assert_eq!(
        unlisted,
        UnlistedAerodrome::new(Some("Farm strip".to_owned()), None).unwrap()
    );
}

#[rstest]
#[case("Farm\nstrip")]
#[case("Farm\u{7}strip")]
fn a_name_with_a_control_character_is_refused(#[case] name: &str) {
    let error = UnlistedAerodrome::new(Some(name.to_owned()), None).unwrap_err();
    assert_eq!(
        error,
        ValidationError::ControlCharacter {
            field: "aerodrome name"
        }
    );
    assert_eq!(
        error.to_string(),
        "aerodrome name must not contain control characters"
    );
}

#[rstest]
#[case("")]
#[case("  \t")]
fn a_blank_name_is_refused(#[case] name: &str) {
    assert_eq!(
        UnlistedAerodrome::new(Some(name.to_owned()), Some(position())).unwrap_err(),
        ValidationError::Empty {
            field: "aerodrome name"
        }
    );
}

#[test]
fn an_unlisted_aerodrome_serialises_as_an_object_without_its_absent_part() {
    let named = Aerodrome::unlisted(Some("Farm strip".to_owned()), None).unwrap();
    assert_eq!(
        serde_json::to_value(&named).unwrap(),
        json!({"name": "Farm strip"})
    );
    let both = Aerodrome::unlisted(Some("Farm strip".to_owned()), Some(position())).unwrap();
    let json = serde_json::to_value(&both).unwrap();
    assert_eq!(
        json,
        json!({"name": "Farm strip", "position": {"latitude": "-33.9461", "longitude": "151.1772"}})
    );
    assert_eq!(serde_json::from_value::<Aerodrome>(json).unwrap(), both);
}

#[test]
fn a_bare_coordinate_is_read_as_a_position_with_no_name() {
    let aerodrome: Aerodrome =
        serde_json::from_value(json!({"latitude": "-33.9461", "longitude": "151.1772"})).unwrap();
    assert_eq!(
        aerodrome,
        Aerodrome::unlisted(None, Some(position())).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&aerodrome).unwrap(),
        json!({"position": {"latitude": "-33.9461", "longitude": "151.1772"}})
    );
}

#[rstest]
#[case(json!({}), "an unlisted aerodrome must have a name or a position")]
#[case(json!({"name": " "}), "aerodrome name must not be empty")]
#[case(json!({"name": "Farm\nstrip"}), "aerodrome name must not contain control characters")]
#[case(
    json!({"latitude": "91", "longitude": "0"}),
    "latitude 91 is out of range (must be -90..=90 degrees)"
)]
#[case(
    json!({"name": "Farm strip", "position": {"latitude": "0", "longitude": "181"}}),
    "longitude 181 is out of range (must be -180..=180 degrees)"
)]
#[case(
    json!({"name": "Farm strip", "elevation": 100}),
    "unknown field `elevation`, expected one of `name`, `position`, `latitude`, `longitude`"
)]
#[case(
    json!({"latitude": "-33.9461", "longitude": "151.1772", "name": "Farm strip"}),
    "an unlisted aerodrome is either {\"name\", \"position\"} or a bare \
     {\"latitude\", \"longitude\"}, not a mix of them"
)]
#[case(
    json!({"latitude": "-33.9461"}),
    "an unlisted aerodrome is either {\"name\", \"position\"} or a bare \
     {\"latitude\", \"longitude\"}, not a mix of them"
)]
#[case(
    json!(42),
    "invalid type: integer `42`, expected a location indicator, or an object describing an unlisted aerodrome"
)]
fn an_invalid_aerodrome_is_refused_on_the_wire_with_its_reason(
    #[case] value: serde_json::Value,
    #[case] reason: &str,
) {
    let error = serde_json::from_value::<Aerodrome>(value).unwrap_err();
    assert_eq!(error.to_string(), reason);
}
