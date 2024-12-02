use chrono::Local;
use chrono_tz::Tz;

use crate::settings::Settings;
use crate::timestamps::*;

pub(crate) fn cmd_now() {
    let ts = Local::now();

    println!("Current time is: {}", ts.format_ts());

    for tz_str in TIMEZONES.iter() {
        let tz: Tz = tz_str.parse().unwrap();
        let converted_ts = ts.with_timezone(&tz);

        println!("Time in {}: {}", tz_str, converted_ts.format_ts(),);
    }
}

pub(crate) fn cmd_validate() {
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
