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
    assert!(serde_json::from_value::<Aerodrome>(json!(value)).is_err());
}

#[rstest]
#[case(Some("Farm strip"), None, "Farm strip")]
#[case(None, Some(position()), "-33.9461,151.1772")]
#[case(Some("Farm strip"), Some(position()), "Farm strip")]
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
#[case(json!({}))] // neither
#[case(json!({"name": " "}))] // a blank name
#[case(json!({"latitude": "91", "longitude": "0"}))] // out of range
#[case(json!({"name": "Farm strip", "elevation": 100}))] // an unknown field
fn an_invalid_unlisted_aerodrome_is_refused_on_the_wire(#[case] value: serde_json::Value) {
    assert!(serde_json::from_value::<Aerodrome>(value).is_err());
}
