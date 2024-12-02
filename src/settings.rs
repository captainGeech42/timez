use chrono_tz::Tz;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct Settings {
    pub(crate) timezones: Vec<String>,
}

impl Settings {
    pub(crate) fn new() -> Result<Settings, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("timez"))
            .build()?;

        s.try_deserialize()
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
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
