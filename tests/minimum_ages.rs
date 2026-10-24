// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The minimum age for each title of licence (Annex 1, 2.3.1.1, 2.4.1.1,
//! 2.5.1.1, 2.6.1.1, 2.9.1.1 and 2.10.1.1), and a State's own in its place.

use icao_shared_kernel::{
    AirlineTransportCategory, IssuedTitle, Licence, LicenceTitle, MinimumAges, Pilot,
    PrivateOrCommercialCategory, ValidationError,
};
use rstest::rstest;
use serde_json::json;
use time::macros::date;
use time::Date;
use uuid::Uuid;

const BORN: Date = date!(2000 - 06 - 15);

fn private_aeroplane() -> LicenceTitle {
    LicenceTitle::Private(PrivateOrCommercialCategory::Aeroplane)
}

fn licence(title: LicenceTitle, issued: Date) -> Licence {
    Licence::new("AU", "CASA", "12345", vec![IssuedTitle::new(title, issued)]).unwrap()
}

fn pilot(born: Date, title: LicenceTitle, issued: Date) -> Result<Pilot, ValidationError> {
    Pilot::with(
        Uuid::new_v4(),
        "Jane Citizen",
        born,
        vec![licence(title, issued)],
    )
}

fn wire(born: Date, title: LicenceTitle, issued: Date) -> serde_json::Value {
    json!({
        "full_name": "Jane Citizen",
        "date_of_birth": born.to_string(),
        "licences": [licence(title, issued)],
    })
}

// Each title is refused the day before the birthday of its minimum age, and
// accepted on that birthday, which counts.
#[rstest]
#[case::private(private_aeroplane(), 17)]
#[case::commercial(LicenceTitle::Commercial(PrivateOrCommercialCategory::Helicopter), 18)]
#[case::multi_crew(LicenceTitle::MultiCrew, 18)]
#[case::airline_transport_aeroplane(
    LicenceTitle::AirlineTransport(AirlineTransportCategory::Aeroplane),
    21
)]
#[case::airline_transport_helicopter(
    LicenceTitle::AirlineTransport(AirlineTransportCategory::Helicopter),
    21
)]
#[case::glider(LicenceTitle::Glider, 16)]
#[case::free_balloon(LicenceTitle::FreeBalloon, 16)]
fn each_title_has_annex_1_minimum_age(#[case] title: LicenceTitle, #[case] years: i32) {
    let birthday = BORN.replace_year(BORN.year() + years).unwrap();
    let day_before = birthday.previous_day().unwrap();
    assert!(matches!(
        pilot(BORN, title, day_before),
        Err(ValidationError::UnderMinimumAge { .. })
    ));
    assert!(pilot(BORN, title, birthday).is_ok());
}

// Born on 29 February 2004: 16 in 2020, a leap year, and 17 in 2021, which is
// not one, so the private pilot licence's 17th year is complete on 1 March.
#[rstest]
#[case::the_day_before_in_a_leap_year(LicenceTitle::Glider, date!(2020 - 02 - 28), false)]
#[case::the_birthday_in_a_leap_year(LicenceTitle::Glider, date!(2020 - 02 - 29), true)]
#[case::the_28th_in_a_common_year(private_aeroplane(), date!(2021 - 02 - 28), false)]
#[case::the_1st_of_march_in_a_common_year(private_aeroplane(), date!(2021 - 03 - 01), true)]
fn a_birth_on_29_february(
    #[case] title: LicenceTitle,
    #[case] issued: Date,
    #[case] accepted: bool,
) {
    assert_eq!(
        pilot(date!(2004 - 02 - 29), title, issued).is_ok(),
        accepted
    );
}

#[test]
fn a_title_issued_before_its_minimum_age_is_refused_with_its_details() {
    let error = pilot(BORN, private_aeroplane(), date!(2017 - 06 - 14)).unwrap_err();
    assert_eq!(
        error,
        ValidationError::UnderMinimumAge {
            issuing_state: "AU".into(),
            issuing_authority: "CASA".into(),
            number: "12345".into(),
            title: private_aeroplane(),
            minimum_age: 17,
            date_of_first_issue: date!(2017 - 06 - 14),
        }
    );
    assert_eq!(
        error.to_string(),
        "private pilot licence (aeroplane) under licence 12345 (CASA, AU) was first issued \
         on 2017-06-14, before the holder was 17"
    );
}

#[test]
fn plain_deserialize_refuses_a_title_below_annex_1_age() {
    let error =
        serde_json::from_value::<Pilot>(wire(BORN, private_aeroplane(), date!(2017 - 06 - 14)))
            .unwrap_err();
    assert_eq!(
        error.to_string(),
        "private pilot licence (aeroplane) under licence 12345 (CASA, AU) was first issued \
         on 2017-06-14, before the holder was 17"
    );
}

// A State whose private pilot licence age is lower than Annex 1's, and every
// other age Annex 1's.
const LOWER: MinimumAges = MinimumAges {
    private: 16,
    commercial: 18,
    multi_crew: 18,
    airline_transport: 21,
    glider: 16,
    free_balloon: 16,
};

#[test]
fn a_lower_age_accepts_what_annex_1_refuses() {
    let issued = date!(2016 - 06 - 15);
    assert!(pilot(BORN, private_aeroplane(), issued).is_err());
    let licences = vec![licence(private_aeroplane(), issued)];
    assert!(
        Pilot::with_minimum_ages(Uuid::new_v4(), "Jane Citizen", BORN, licences, &LOWER).is_ok()
    );
}

#[test]
fn a_lower_age_reads_what_annex_1_refuses() {
    let json = wire(BORN, private_aeroplane(), date!(2016 - 06 - 15));
    assert!(serde_json::from_value::<Pilot>(json.clone()).is_err());
    assert!(Pilot::deserialize_with(json, &LOWER).is_ok());
}

#[test]
fn a_higher_age_refuses_what_annex_1_accepts() {
    let ages = MinimumAges {
        commercial: 19,
        ..MinimumAges::default()
    };
    let title = LicenceTitle::Commercial(PrivateOrCommercialCategory::Aeroplane);
    let issued = date!(2018 - 06 - 15);
    assert!(pilot(BORN, title, issued).is_ok());
    let licences = vec![licence(title, issued)];
    let error = Pilot::with_minimum_ages(Uuid::new_v4(), "Jane Citizen", BORN, licences, &ages)
        .unwrap_err();
    assert!(matches!(
        error,
        ValidationError::UnderMinimumAge {
            minimum_age: 19,
            ..
        }
    ));
    let read = Pilot::deserialize_with(wire(BORN, title, issued), &ages).unwrap_err();
    assert_eq!(
        read.to_string(),
        "commercial pilot licence (aeroplane) under licence 12345 (CASA, AU) was first issued \
         on 2018-06-15, before the holder was 19"
    );
}
