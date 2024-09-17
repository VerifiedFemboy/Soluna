use chrono::Timelike;

mod calculations;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        std::thread::sleep(std::time::Duration::from_millis(500));
        let current_time: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let julian_day = calculations::julian_day(current_time);
        let time = current_time.hour() as f64 + current_time.minute() as f64 / 60.0 + current_time.second() as f64 / 3600.0;
        let longitude = 15.0;

        let declination = calculations::solar_declination(julian_day);
        let hour_angle = calculations::solar_hour_angle(julian_day, time, longitude);

        println!("Current Time (UTC): {}", current_time);
        println!("Julian Day: {}", julian_day);
        println!("Declination: {}", declination);
        println!("Hour Angle: {}", hour_angle);
    }
}
