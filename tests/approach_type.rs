// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Approach types: which are 3D approach operations, and how they serialise.

use icao_shared_kernel::{ApproachType, RnpApchMinima};

#[test]
fn an_approach_is_3d_when_it_has_vertical_guidance() {
    use ApproachType::*;
    use RnpApchMinima::*;
    for (approach, is_3d) in [
        (Ndb, false),
        (Vor, false),
        (Localizer, false),
        (Ils, true),
        (Gls, true),
        (RnpAr, true),
        (RnpApch { minima: Lnav }, false),
        (RnpApch { minima: Lp }, false),
        (RnpApch { minima: LnavVnav }, true),
        (RnpApch { minima: Lpv }, true),
    ] {
        assert_eq!(approach.is_3d(), is_3d, "{approach:?}");
    }
}

#[test]
fn approach_types_are_spelled_as_their_names_and_read_back() {
    use ApproachType::*;
    for (approach, json) in [
        (Ndb, r#""ndb""#),
        (RnpAr, r#""rnp-ar""#),
        (
            RnpApch {
                minima: RnpApchMinima::LnavVnav,
            },
            r#"{"rnp-apch":{"minima":"lnav-vnav"}}"#,
        ),
    ] {
        assert_eq!(serde_json::to_string(&approach).unwrap(), json);
        assert_eq!(
            serde_json::from_str::<ApproachType>(json).unwrap(),
            approach
        );
    }
}
