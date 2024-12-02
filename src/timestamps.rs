use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};

pub(crate) trait TimestampString {
    /// Convert the UTC timestamp to a consistent string representation.
    fn format_ts(&self) -> String;
}

/// Format string for TZ-aware chrono timestamp objects.
// specifiers: https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html
const TZ_FSTR: &str = "%Y-%m-%d %H:%M:%S %Z";

/// Format string for TZ-naive chrono timestamp objects.
const NTZ_FSTR: &str = "%Y-%m-%d %H:%M:%S";

impl TimestampString for DateTime<Utc> {
    fn format_ts(&self) -> String {
        return self.format(TZ_FSTR).to_string();
    }
}

impl TimestampString for DateTime<Local> {
    fn format_ts(&self) -> String {
        return self.format(TZ_FSTR).to_string();
    }
}

impl TimestampString for NaiveDateTime {
    fn format_ts(&self) -> String {
        return self.format(NTZ_FSTR).to_string();
    }
}

impl TimestampString for DateTime<Tz> {
    fn format_ts(&self) -> String {
        let offset = self
            .timezone()
            .offset_from_local_datetime(&self.naive_local())
            .unwrap();

        // Not all timezones have an abbreviation defined, so fallback to the hours delta (eg, `+0430`)
        // if an abbreviation isn't available.
        // Example: `Asia/Kabul`
        let abbr = match offset.abbreviation() {
            Some(x) => x.to_string(),
            None => offset.to_string(),
        };

        return format!("{} {}", self.format(NTZ_FSTR).to_string(), abbr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_dt_utc_ts() {
        let ts = Utc.with_ymd_and_hms(2024, 10, 30, 16, 08, 42).unwrap();
        assert_eq!(ts.format_ts(), "2024-10-30 16:08:42 UTC");
    }

    #[test]
    fn verify_dt_local_ts() {
        let ts = Local.with_ymd_and_hms(2024, 10, 30, 16, 08, 42).unwrap();

        // The offset is dependent on the running machine's local timezone, so can't reliably hardcode it.
        let offset = ts.offset().to_string();

        assert_eq!(ts.format_ts(), format!("2024-10-30 16:08:42 {offset}"));
    }

    #[test]
    fn verify_ndt_ts() {
        use chrono::NaiveDate;

        let ts = NaiveDate::from_ymd_opt(2024, 10, 30)
            .unwrap()
            .and_hms_opt(16, 08, 42)
            .unwrap();
        assert_eq!(ts.format_ts(), "2024-10-30 16:08:42");
    }

    #[test]
    fn verify_dt_tz_ts_gmt() {
        use chrono_tz::Europe::London;

        let ts = London.from_utc_datetime(
            &Utc.with_ymd_and_hms(2024, 10, 30, 16, 08, 42)
                .unwrap()
                .naive_utc(),
        );

        assert_eq!(ts.format_ts(), "2024-10-30 16:08:42 GMT");
    }

    #[test]
    fn verify_dt_tz_ts_cst() {
        use chrono_tz::Asia::Chongqing;

        let ts = Chongqing.from_utc_datetime(
            &Utc.with_ymd_and_hms(2024, 10, 30, 16, 08, 42)
                .unwrap()
                .naive_utc(),
        );

        assert_eq!(ts.format_ts(), "2024-10-31 00:08:42 CST");
    }

    #[test]
    fn verify_dt_tz_ts_offset() {
        use chrono_tz::Asia::Kabul;

        let ts = Kabul.from_utc_datetime(
            &Utc.with_ymd_and_hms(2024, 10, 30, 16, 08, 42)
                .unwrap()
                .naive_utc(),
        );

        assert_eq!(ts.format_ts(), "2024-10-30 20:38:42 +0430");
    }
}
