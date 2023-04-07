//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/xxxxxxxxxxxxx
//
//=================================================

use chrono::{Datelike, Timelike, Utc};
use env_logger::{fmt::Color, Env};
use std::io::Write;

pub(crate) fn env_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(move |buf, record| {
            let now = Utc::now();
            let (_is_common_era, year) = now.year_ce();
            let (_is_pm, hour) = now.hour12();
            let mut level_style = buf.style();

            if format!("{}", record.level()) == "DEBUG" {
                level_style.set_color(Color::Rgb(80, 80, 80)).set_bold(true);
            } else if format!("{}", record.level()) == "ERROR" {
                level_style.set_color(Color::Red).set_bold(true);
            } else if format!("{}", record.level()) == "WARN" {
                level_style.set_color(Color::Yellow).set_bold(true);
            }

            writeln!(
                buf,
                "[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [{}]: {}",
                year,
                now.month(),
                now.day(),
                now.weekday(),
                hour,
                now.minute(),
                now.second(),
                level_style.value(record.level()),
                record.args()
            )
        })
        .init();
}
