use core::panic;

use ethiopic_calendar::{EthiopianYear, GregorianYear};
use pgrx::{prelude::*, AnyElement};

::pgrx::pg_module_magic!();

#[pg_extern]
fn ethopic(any: AnyElement) -> Option<String> {
    let date = match any.oid() {
        pg_sys::DATEOID => any.datum().try_into().ok().map(ethopic_from_date),
        pg_sys::TIMESTAMPOID => any.datum().try_into().ok().map(ethopic_from_timestamp),
        pg_sys::TIMESTAMPTZOID => any.datum().try_into().ok().map(ethopic_from_timestampz),
        _ => None,
    };

    date.map(to_string)
}

fn ethopic_from_date(date: Date) -> EthiopianYear {
    GregorianYear::new(
        date.year() as usize,
        date.month() as usize,
        date.day() as usize,
    )
    .into()
}

fn ethopic_from_timestamp(date: Timestamp) -> EthiopianYear {
    GregorianYear::new(
        date.year() as usize,
        date.month() as usize,
        date.day() as usize,
    )
    .into()
}

fn ethopic_from_timestampz(date: TimestampWithTimeZone) -> EthiopianYear {
    GregorianYear::new(
        date.year() as usize,
        date.month() as usize,
        date.day() as usize,
    )
    .into()
}

fn to_string(date: EthiopianYear) -> String {
    let day = date.day();
    let month = date.amharic_month();
    let year = date.formatted_year();

    format!("{month} {day}, {year}")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_ethopic() {
        assert_eq!("", "");
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
