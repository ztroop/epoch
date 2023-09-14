extern crate chrono;
extern crate clap;

use chrono::{FixedOffset, Local, NaiveDateTime, TimeZone};
use clap::{App, Arg};
use std::str::FromStr;

fn main() {
    let matches = App::new("Epoch to Time Converter")
        .version("0.1")
        .author("Zackary Troop <zackary.troop@outlook.com>")
        .about("Converts epoch to local time and vice versa.")
        .arg(
            Arg::with_name("epoch")
                .short("e")
                .long("epoch")
                .value_name("EPOCH")
                .help("Converts an epoch timestamp to local time")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("date_string")
                .short("d")
                .long("date")
                .value_name("DATE_STRING")
                .help("Converts a date string to epoch timestamp")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Specifies the format to parse the date string")
                .takes_value(true)
                .requires("date_string"),
        )
        .arg(
            Arg::with_name("timezone")
                .short("z")
                .long("timezone")
                .value_name("UTC_OFFSET")
                .help("Specifies the timezone as UTC±N, where N is the hour offset from UTC.")
                .takes_value(true),
        )
        .get_matches();

    let offset_sec: i32 = if let Some(tz_str) = matches.value_of("timezone") {
        if tz_str.starts_with("UTC") {
            if let Ok(n) = i32::from_str(&tz_str[4..]) {
                let sign = if &tz_str[3..4] == "+" { 1 } else { -1 };
                sign * n * 60 * 60
            } else {
                println!("Invalid timezone offset");
                return;
            }
        } else {
            println!("Invalid timezone format. Use UTC±N.");
            return;
        }
    } else {
        Local::now().offset().local_minus_utc()
    };

    // Convert epoch to local time (or to the specified timezone)
    if let Some(epoch_str) = matches.value_of("epoch") {
        if let Ok(epoch) = epoch_str.parse::<i64>() {
            let adjusted_epoch = epoch + offset_sec as i64;
            match NaiveDateTime::from_timestamp_opt(adjusted_epoch, 0) {
                Some(naive_datetime) => match FixedOffset::east_opt(offset_sec) {
                    Some(fixed_offset) => {
                        let datetime_with_tz = fixed_offset.from_utc_datetime(&naive_datetime);
                        println!("{datetime_with_tz}");
                    }
                    None => {
                        println!("Invalid timezone offset.");
                    }
                },
                None => {
                    println!("Invalid or out-of-bounds epoch time.");
                }
            }
        } else {
            println!("Invalid epoch input");
        }
    }

    // Convert date string to epoch (or to the specified timezone)
    if let Some(date_str) = matches.value_of("date_string") {
        if let Some(format_str) = matches.value_of("format") {
            match NaiveDateTime::parse_from_str(date_str, format_str) {
                Ok(naive_datetime) => {
                    match FixedOffset::east_opt(offset_sec) {
                        Some(fixed_offset) => {
                            // Convert to DateTime<FixedOffset>
                            let datetime_with_tz = fixed_offset.from_utc_datetime(&naive_datetime);
                            // Convert to epoch, considering the timezone
                            let epoch_with_tz = datetime_with_tz.timestamp() - offset_sec as i64;
                            println!("{epoch_with_tz}");
                        }
                        None => {
                            println!("Invalid timezone offset.");
                        }
                    }
                }
                Err(_) => {
                    println!("Failed to parse date string with given format");
                }
            }
        } else {
            println!("Format string is required when converting from date string");
        }
    }
}
