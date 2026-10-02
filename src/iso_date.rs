//! A date on the wire as ISO 8601's `YYYY-MM-DD`, as people write it, in place
//! of `time`'s compact `[year, ordinal]` default.

time::serde::format_description!(pub(crate) iso_date, Date, "[year]-[month]-[day]");

pub(crate) use iso_date::{deserialize, serialize};
