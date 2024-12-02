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
