use chrono::{Local, Utc};
use chrono_tz::Tz;

use crate::settings::Settings;
use crate::timestamps::{parse, TimestampString};

pub(crate) fn cmd_now() {
    let s = Settings::new().unwrap();

    let ts = Local::now();

    println!("Current local time is: {}", ts.format_ts());

    for tz_str in s.timezones.iter() {
        let tz: Tz = tz_str.parse().unwrap();
        let converted_ts = ts.with_timezone(&tz);

        println!("Time in {}: {}", tz_str, converted_ts.format_ts(),);
    }
}

pub(crate) fn cmd_convert(value: String) {
    let s = Settings::new().unwrap();

    let ts = parse(value).unwrap();
    for tz_str in s.timezones.iter() {
        let tz: Tz = tz_str.parse().unwrap();
        let converted_ts = ts.with_timezone(&tz);

        println!("Time in {}: {}", tz_str, converted_ts.format_ts(),);
    }
}

pub(crate) fn cmd_as(timezone: String) {
    let tz = match timezone.parse::<Tz>() {
        Ok(tz) => tz,
        Err(e) => {
            eprintln!("invalid timezone: {e}");
            std::process::exit(1);
        }
    };

    let ts = Utc::now().with_timezone(&tz);
    println!("{}", ts.format_ts());
}

pub(crate) fn cmd_validate() {
    let s = Settings::new().unwrap();

    match s.validate() {
        Ok(_) => {
            println!("config is valid");
        }
        Err(e) => {
            eprintln!("config is invalid!");
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
