// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The titles of licence held under a licence, and their dates of first issue.

use icao_shared_kernel::{
    AircraftCategory, IssuedTitle, Licence, LicenceTitle, PrivateOrCommercialCategory,
    ValidationError,
};
use rstest::rstest;
use serde_json::json;
use time::macros::date;

fn private_aeroplane() -> LicenceTitle {
    LicenceTitle::Private(PrivateOrCommercialCategory::Aeroplane)
}

#[test]
fn holds_each_title_from_its_own_date() {
    // Annex 1, 2.1.1.2.1: one licence can carry more than one licence's title.
    let licence = Licence::new(
        "AU",
        "CASA",
        "12345",
        vec![
            IssuedTitle::new(private_aeroplane(), date!(2015 - 04 - 01)),
            IssuedTitle::new(
                LicenceTitle::Commercial(PrivateOrCommercialCategory::Helicopter),
                date!(2019 - 08 - 20),
            ),
        ],
    )
    .unwrap();
    let dates: Vec<_> = licence
        .titles()
        .iter()
        .map(|issued| (issued.title.category(), issued.date_of_first_issue))
        .collect();
    assert_eq!(
        dates,
        [
            (AircraftCategory::Aeroplane, date!(2015 - 04 - 01)),
            (AircraftCategory::Helicopter, date!(2019 - 08 - 20)),
        ]
    );
}

#[test]
fn a_licence_may_hold_no_annex_1_title() {
    // Its titles may all be a State's own, which are kept downstream.
    assert!(Licence::new("AU", "CASA", "12345", vec![]).is_ok());
}

#[test]
fn a_title_listed_twice_is_refused() {
    let error = Licence::new(
        "AU",
        "CASA",
        "12345",
        vec![
            IssuedTitle::new(private_aeroplane(), date!(2015 - 04 - 01)),
            IssuedTitle::new(private_aeroplane(), date!(2016 - 04 - 01)),
        ],
    )
    .unwrap_err();
    assert_eq!(
        error,
        ValidationError::RepeatedLicenceTitle(private_aeroplane())
    );
    assert_eq!(
        error.to_string(),
        "private pilot licence (aeroplane) is listed more than once"
    );
}

#[test]
fn a_licence_without_titles_is_refused() {
    let error = serde_json::from_value::<Licence>(json!({
        "issuing_state": "AU", "issuing_authority": "CASA", "number": "12345"
    }))
    .unwrap_err();
    assert!(error.to_string().contains("titles"), "{error}");
}

#[test]
fn a_title_without_a_date_of_first_issue_is_refused() {
    let error = serde_json::from_value::<IssuedTitle>(json!({
        "licence": "private", "category": "aeroplane"
    }))
    .unwrap_err();
    assert!(error.to_string().contains("date_of_first_issue"), "{error}");
}

#[test]
fn a_licence_round_trips_with_its_titles() {
    let wire = json!({
        "issuing_state": "AU",
        "issuing_authority": "CASA",
        "number": "12345",
        "titles": [
            {"licence": "private", "category": "aeroplane", "date_of_first_issue": "2015-04-01"}
        ]
    });
    let licence: Licence = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(licence.titles()[0].title, private_aeroplane());
    assert_eq!(serde_json::to_value(licence).unwrap(), wire);
}

// Annex 1: 2.3.1 and 2.4.1 (private and commercial), 2.5 (multi-crew), 2.6.3
// to 2.6.5 (airline transport), 2.9 (glider), 2.10 (free balloon).
#[rstest]
#[case("private", "aeroplane")]
#[case("private", "airship")]
#[case("private", "helicopter")]
#[case("private", "powered-lift")]
#[case("commercial", "aeroplane")]
#[case("commercial", "airship")]
#[case("commercial", "helicopter")]
#[case("commercial", "powered-lift")]
#[case("multi-crew", "aeroplane")]
#[case("airline-transport", "aeroplane")]
#[case("airline-transport", "helicopter")]
#[case("airline-transport", "powered-lift")]
#[case("glider", "glider")]
#[case("free-balloon", "free-balloon")]
fn the_titles_annex_1_issues_are_read(#[case] licence: &str, #[case] category: &str) {
    let wire =
        json!({"licence": licence, "category": category, "date_of_first_issue": "2015-04-01"});
    let issued: IssuedTitle = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(issued).unwrap(), wire);
}

#[rstest]
#[case("private", "glider")]
#[case("private", "free-balloon")]
#[case("commercial", "glider")]
#[case("commercial", "free-balloon")]
#[case("multi-crew", "helicopter")]
#[case("multi-crew", "airship")]
#[case("airline-transport", "airship")]
#[case("airline-transport", "glider")]
#[case("glider", "aeroplane")]
#[case("free-balloon", "airship")]
fn a_title_annex_1_does_not_issue_is_refused(#[case] licence: &str, #[case] category: &str) {
    let wire =
        json!({"licence": licence, "category": category, "date_of_first_issue": "2015-04-01"});
    let error = serde_json::from_value::<IssuedTitle>(wire).unwrap_err();
    assert!(error.to_string().contains("is not issued for"), "{error}");
}

#[test]
fn a_title_annex_1_does_not_issue_is_named_in_full() {
    let wire = json!({"licence": "airline-transport", "category": "airship", "date_of_first_issue": "2015-04-01"});
    let error = serde_json::from_value::<IssuedTitle>(wire).unwrap_err();
    assert_eq!(
        error.to_string(),
        "the airline transport pilot licence is not issued for the airship category"
    );
}

#[test]
fn the_order_titles_are_listed_in_does_not_matter() {
    let private = IssuedTitle::new(private_aeroplane(), date!(2015 - 04 - 01));
    let glider = IssuedTitle::new(LicenceTitle::Glider, date!(2016 - 04 - 01));
    let licence = |titles| Licence::new("AU", "CASA", "12345", titles).unwrap();
    assert_eq!(
        licence(vec![private.clone(), glider.clone()]),
        licence(vec![glider, private])
    );
}
