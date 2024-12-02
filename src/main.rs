use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use clap::{Parser, Subcommand};
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current time in the configured timezones.
    Now,

    /// Inspect the config file to confirm it is valid.
    Validate,
}

trait TimestampString {
    /// Convert the UTC timestamp to a consistent string representation.
    fn format_ts(&self) -> String;
}

/// Format string for TZ-aware chrono timestamp objects.
// specifiers: https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html
const TZ_FSTR: &str = "%Y-%m-%d %H:%M:%S %Z";

/// Format string for TZ-naive chrono timestamp objects.
const NTZ_FSTR: &str = "%Y-%m-%d %H:%M:%S";

/// List of timezones to show.
const TIMEZONES: &[&str] = &["Europe/London", "America/Chicago", "Asia/Kabul"];

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

fn cmd_now() {
    let ts = Local::now();

    println!("Current time is: {}", ts.format_ts());

    for tz_str in TIMEZONES.iter() {
        let tz: Tz = tz_str.parse().unwrap();
        let converted_ts = ts.with_timezone(&tz);

        println!("Time in {}: {}", tz_str, converted_ts.format_ts(),);
    }
}

fn cmd_validate() {
    let s = Settings::new().unwrap();

    match s.validate() {
        Ok(_) => {
            println!("config is valid");
        }
        Err(e) => {
            println!("config is invalid!");
            println!("{e}");
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Settings {
    timezones: Vec<String>,
}

impl Settings {
    fn new() -> Result<Settings, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("timez"))
            .build()?;

        s.try_deserialize()
    }

    fn validate(&self) -> Result<(), String> {
        // make sure that each timezone name is valid
        for tz_str in self.timezones.iter() {
            let tz = tz_str.parse::<Tz>();
            match tz {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "the timezone string '{}' is not valid: {}",
                        tz_str, e
                    ))
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Now) => cmd_now(),
        Some(Commands::Validate) => cmd_validate(),
        None => {}
    }
}
