use chrono::{DateTime, Datelike, Timelike, Utc};

pub fn solar_declination(julian_day: f64) -> f64 {
    let n = julian_day - 1.0;
    let angle = 23.44 * (std::f64::consts::PI / 180.0);
    let declination = angle * (360.0 * (n + 10.0) / 365.0).cos();
    declination
}

pub fn solar_hour_angle(julian_day: f64, time: f64, longitude: f64) -> f64 {
    let n = julian_day - 1.0;
    let angle = 23.44 * (std::f64::consts::PI / 180.0);
    let declination = angle * (360.0 * (n + 10.0) / 365.0).cos();
    let hour_angle = (time - 12.0) * 15.0 + longitude;
    hour_angle
}

pub fn julian_day(utc: DateTime<Utc>) -> f64 {
    let year = utc.year();
    let month = utc.month();
    let day = utc.day();
    let hour = utc.hour();
    let minute = utc.minute();
    let second = utc.second();
    let millisecond = utc.timestamp_subsec_millis();
    let millisecond = millisecond as f64 / 1000.0;
    let julian_day = 367.0 * year as f64 - (7.0 * (year as f64 + ((month as f64 + 9.0) / 12.0)) / 4.0).floor() + (275.0 * month as f64 / 9.0).floor() + day as f64 + 1721013.5 + ((hour as f64 + (minute as f64 + (second as f64 + millisecond) / 60.0) / 60.0) / 24.0);
    julian_day
}
