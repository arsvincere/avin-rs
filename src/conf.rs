/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{
    DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta,
    TimeZone, Utc,
};

use log::{Metadata, Record};

// TODO: move to separate mod
// log
pub static LOGGER: ConsoleLogger = ConsoleLogger;
pub struct ConsoleLogger;
impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // metadata.level() <= log::Level::Info
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} [{}] {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

// GUI
pub const APP_ID: &str = "com.arsvincere.AVIN";

// Settings
pub const DEFAULT_BARS_COUNT: i32 = 5000;
pub const DEFAULT_COMMISSION: f64 = 0.0005;

// Dir
pub const ASSET_DIR: &str = "/home/alex/avin/usr/asset";
pub const CACHE_DIR: &str = "/home/alex/avin/usr/cache";
pub const DATA_DIR: &str = "/home/alex/avin/usr/data";
pub const TEST_DIR: &str = "/home/alex/avin/usr/test";

// Connect
pub const TINKOFF_TOKEN: &str =
    "/home/alex/avin/usr/connect/tinkoff/token.txt";

// Datetime
pub const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";
pub const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
pub const DAY_END: NaiveTime = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
pub const MSK_TIME_DIF: TimeDelta = TimeDelta::hours(3);
pub const MINUTES_IN_DAY: i32 = 24 * 60 * 60;

// TODO: move to utils
pub struct Usr {}
impl Usr {
    /// Return UTC datetime from user local datetime
    pub fn dt(dt: &str) -> DateTime<Utc> {
        let dt = NaiveDateTime::parse_from_str(dt, DT_FMT).unwrap();
        let dt = Local.from_local_datetime(&dt).unwrap();

        dt.to_utc()
    }
    /// Return UTC datetime from user local date
    pub fn date(d: &str) -> DateTime<Utc> {
        let dt = NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::MIN);
        let dt = Local.from_local_datetime(&dt).unwrap();

        dt.to_utc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dt() {
        let dt = Usr::dt("2025-01-01 10:00:00");
        let utc_dt = Utc.with_ymd_and_hms(2025, 1, 1, 7, 0, 0).unwrap();
        assert_eq!(utc_dt, dt);
    }
    #[test]
    fn date() {
        let dt = Usr::date("2025-01-01");
        let utc_dt = Utc.with_ymd_and_hms(2024, 12, 31, 21, 0, 0).unwrap();
        assert_eq!(utc_dt, dt);
    }
}
