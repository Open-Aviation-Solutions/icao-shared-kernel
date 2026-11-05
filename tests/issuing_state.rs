// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The issuing State: an ISO 3166-1 alpha-2 code, by shape.

use icao_shared_kernel::{DeviceQualification, IssuingState, Licence, ValidationError};
use rstest::rstest;
use serde_json::json;

#[test]
fn a_two_letter_code_is_accepted() {
    assert_eq!(IssuingState::parse("AU").unwrap().as_str(), "AU");
}

#[rstest]
#[case("australia")] // not a code
#[case("AUS")] // three letters
#[case("A")] // one letter
#[case("au")] // lowercase
#[case("A1")] // a digit
#[case("")] // empty
fn a_malformed_code_is_refused(#[case] state: &str) {
    assert_eq!(
        IssuingState::parse(state),
        Err(ValidationError::IssuingState(state.to_owned()))
    );
}

#[test]
fn it_serialises_as_the_plain_code() {
    let state: IssuingState = serde_json::from_value(json!("AU")).unwrap();
    assert_eq!(serde_json::to_value(state).unwrap(), json!("AU"));
    assert!(serde_json::from_value::<IssuingState>(json!("au")).is_err());
}

#[test]
fn a_licence_with_a_malformed_issuing_state_is_refused() {
    let error = serde_json::from_value::<Licence>(json!({
        "issuing_state": "au", "issuing_authority": "CASA", "number": "12345", "titles": []
    }))
    .unwrap_err();
    assert!(error.to_string().contains("ISO 3166-1"), "{error}");
}

#[test]
fn a_qualification_with_a_malformed_issuing_state_is_refused() {
    let error = serde_json::from_value::<DeviceQualification>(json!({
        "issuing_state": "au", "issuing_authority": "CASA", "level": "Level D",
        "valid_from": "2026-01-01", "valid_until": "2026-12-31"
    }))
    .unwrap_err();
    assert!(error.to_string().contains("ISO 3166-1"), "{error}");
}
