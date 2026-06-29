//! Waypoint validation parity — mirrors `tests/models/flight/test_flight.py`.

use icao_shared_kernel::{DomainError, Waypoint};

#[test]
fn valid_codes_accepted() {
    for value in ["YSBK", "AB", "ABCDE", "RIVET", "DCT01", "WAY99"] {
        let waypoint = Waypoint::parse(value).expect("should be valid");
        assert_eq!(waypoint.as_str(), value);
    }
}

#[test]
fn invalid_codes_rejected() {
    // lowercase, mixed-case, length 1, length 6, empty, non-alphanumeric
    for value in ["ybth", "Ybth", "A", "ABCDEF", "", "YS-BK"] {
        assert!(
            Waypoint::parse(value).is_err(),
            "{value:?} should be rejected"
        );
    }
}

#[test]
fn error_messages_match_python_kernel() {
    assert_eq!(
        Waypoint::parse("A").unwrap_err().to_string(),
        "Waypoint 'A' must be 2-5 characters long",
    );
    assert_eq!(
        Waypoint::parse("ybth").unwrap_err().to_string(),
        "Waypoint 'ybth' must be uppercase alphanumeric",
    );
}

#[test]
fn all_digit_code_rejected() {
    // Faithful to Python `isupper()`: a code with no cased letters is rejected.
    assert!(matches!(
        Waypoint::parse("12345"),
        Err(DomainError::WaypointCharset(_))
    ));
}
