// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The location indicator: four uppercase letters, the shape of ICAO Doc
//! 7910's.

use icao_shared_kernel::{LocationIndicator, ValidationError};
use rstest::rstest;
use serde_json::json;

#[test]
fn four_uppercase_letters_are_accepted() {
    assert_eq!(LocationIndicator::parse("YSBK").unwrap().as_str(), "YSBK");
}

#[test]
fn it_compares_with_the_plain_code() {
    let indicator = LocationIndicator::parse("YSBK").unwrap();
    let held: &LocationIndicator = &indicator; // as a getter returns it
    assert!(held == "YSBK");
    assert!(indicator == *"YSBK");
    assert!(indicator != "KJFK");
    assert_eq!(indicator.as_ref(), "YSBK");
}

#[rstest]
#[case("YSB")] // three letters
#[case("YSBKX")] // five letters
#[case("ysbk")] // lowercase
#[case("YS1K")] // a digit
#[case("YSÄK")] // not ASCII
#[case("")] // empty
fn anything_else_is_refused(#[case] value: &str) {
    let error = LocationIndicator::parse(value).unwrap_err();
    assert_eq!(error, ValidationError::LocationIndicator(value.to_owned()));
    assert_eq!(
        error.to_string(),
        format!("location indicator '{value}' must be four uppercase letters")
    );
}

#[test]
fn it_serialises_as_the_plain_code() {
    let indicator: LocationIndicator = serde_json::from_value(json!("YSBK")).unwrap();
    assert_eq!(serde_json::to_value(indicator).unwrap(), json!("YSBK"));
    assert!(serde_json::from_value::<LocationIndicator>(json!("ysbk")).is_err());
}
