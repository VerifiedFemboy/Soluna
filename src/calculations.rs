use std::f64::consts::PI;

use chrono::{DateTime, Datelike, Local, Timelike};

pub fn solar_declination(current_day_of_year: f64) -> f64 {
    //It calculates the solar declination angle orbital tilt of the Earth
    let gamma = (-23.45_f64).to_radians().sin() * ((360.0 / 365.0) * (current_day_of_year + 10.0)).to_radians().cos();
    gamma
}

pub fn solar_hour_angle(time: &f64, longitude: &f64) -> f64 {
    let hour_angle = (time - 12.0) * 15.0 + longitude;
    hour_angle
}

pub fn calculate_julian_day(local: DateTime<Local>) -> f64 {
    let year = local.year();
    let month = local.month();
    let day = local.day();
    let hour = local.hour();
    let minute = local.minute();
    let second = local.second();
    let millisecond = local.timestamp_subsec_millis();
    let millisecond = millisecond as f64 / 1000.0;
    let julian_day = 367.0 * year as f64 - (7.0 * (year as f64 + ((month as f64 + 9.0) / 12.0)) / 4.0).floor() + (275.0 * month as f64 / 9.0).floor() + day as f64 + 1721013.5 + ((hour as f64 + (minute as f64 + (second as f64 + millisecond) / 60.0) / 60.0) / 24.0);
    julian_day
}

pub fn solar_ecliptic_position(day_of_year: f64) -> f64 {
    
    let l0 = 280.46; //in degrees
    
    let days_since_march = day_of_year - 80.0; //March 21st is the 80th day of the year
    
    let daily_angle = 360.0 / 365.25; //365.25 days in a year
    
    let mut lambda_s = l0 + daily_angle * days_since_march; //in degrees
    
    if lambda_s > 360.0 {
        lambda_s -= 360.0;
    } else if lambda_s < 0.0 {
        lambda_s += 360.0;
    }
    
    lambda_s
}

pub fn moon_position(n: f64) -> (f64, f64) {
    
    // Average of the moon's length
    let l_m = 218.316 + 13.176396 * n;
    // Average of anomaly of the moon
    let m_m = 134.963 + 13.064993 * n;

    // Average of width of the moon
    let f = 93.272 + 13.229350 * n;

    // Convert to radians
    let m_m_radiands = m_m.to_radians();
    let f_radians = f.to_radians();

    // Calculate the moon's position
    let lambda_m = l_m + 6.289 * m_m_radiands.sin();
    let beta_m = 5.128 * f_radians.sin();

    (lambda_m, beta_m)
}

pub fn moon_phase(julian_day: &f64) -> f64 {
    let days_since_new_moon = julian_day - 2451550.1; // Reference new moon date: January 6, 2000
    let new_moon_cycle = 29.53058867; // Average length of the lunar cycle in days
    let phase = (days_since_new_moon % new_moon_cycle) / new_moon_cycle;
    phase
}

pub fn moon_phase_as_str(julian_day: &f64) -> String {
    let phase = moon_phase(julian_day);
    let percentage = illumination(phase);
    let phase_str = match phase {
        p if p < 0.03 => "🌑 New Moon",
        p if p < 0.25 => "🌒 Waxing Crescent",
        p if p < 0.27 => "🌓 First Quarter",
        p if p < 0.50 => "🌔 Waxing Gibbous",
        p if p < 0.53 => "🌕 Full Moon",
        p if p < 0.75 => "🌖 Waning Gibbous",
        p if p < 0.77 => "🌗 Last Quarter",
        _ => "🌘 Waning Crescent",
    };
    phase_str.to_string() + &format!(" ({:.2}%)", percentage)
}

pub fn illumination(phase: f64) -> f64 {
    let result = 50.0 * (1.0 - (2.0 * PI * phase).cos());
    result
}
//TODO: make better calculations
pub fn next_full_moon(julian_day: &f64) -> i8 {
    let new_moon_cycle = 29.53058867; // Average length of the lunar cycle in days
    let phase = moon_phase(julian_day);
    let days_until_full_moon = new_moon_cycle * (0.5 - phase);
    (julian_day + days_until_full_moon) as i8
}