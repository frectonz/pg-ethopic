use ethiopic_calendar::{EthiopianYear, GregorianYear};
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[pg_extern]
fn ethopic_from_date(date: Date) -> String {
    let date: EthiopianYear = GregorianYear::new(
        date.year() as usize,
        date.month() as usize,
        date.day() as usize,
    )
    .into();

    let month = date.amharic_month();
    let day = date.day();
    let year = date.formatted_year();

    format!("{month} {day}, {year}")
}

#[pg_extern]
fn ethopic_from_timestamp(date: TimestampWithTimeZone) -> String {
    let date: EthiopianYear = GregorianYear::new(
        date.year() as usize,
        date.month() as usize,
        date.day() as usize,
    )
    .into();

    let month = date.amharic_month();
    let day = date.day();
    let year = date.formatted_year();

    format!("{month} {day}, {year}")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_ethopic() {
        let date = Date::new(2004, 3, 29).unwrap();
        assert_eq!("መጋቢት 20, 1996", crate::ethopic_from_date(date));
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
