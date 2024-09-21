use chrono::Timelike;

mod calculations;

#[tokio::main]
async fn main() {

    let current_ip = reqwest::get("https://api64.ipify.org?format=json")
    .await
    .unwrap().text().await.unwrap();

    let current_ip: serde_json::Value = serde_json::from_str(&current_ip).unwrap();

    let geolocation = geolocation::find(current_ip["ip"].as_str().unwrap()).unwrap();
    let longitude = geolocation.longitude.parse::<f64>().unwrap();
    let latitude = geolocation.latitude.parse::<f64>().unwrap();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        std::thread::sleep(std::time::Duration::from_millis(100));
        let current_time: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let julian_day = calculations::calculate_julian_day(current_time);
        let time = current_time.hour() as f64 + current_time.minute() as f64 / 60.0 + current_time.second() as f64 / 3600.0;

        let solar_declination = calculations::solar_declination(julian_day);
        let solar_hour_angle = calculations::solar_hour_angle(time, longitude);
        let solar_ecliptic_position = calculations::solar_ecliptic_position(julian_day - 1721013.5);

        let current_time_format: String = current_time.format("%H:%M:%S | %m-%d-%Y").to_string();
    

        println!("Current Time (LOCAL): {}", current_time_format);
        println!("  Location Information");
        println!("      Latitude: {}", latitude);
        println!("      Longitude: {}", longitude);
        println!("      Timezone: {}", geolocation.timezone);
        println!("      City: {}", geolocation.city);
        println!("      Region: {}", geolocation.region);
        println!("      Country: {}", geolocation.country);
        println!("");
        println!("  Calculated Information");
        println!("    Sun:");
        println!("      Julian Day: {}", julian_day);
        println!("      Solar Declination: {}", solar_declination);
        println!("      Solar Hour Angle: {}", solar_hour_angle);
        println!("      Solar Ecliptic Position: {}", solar_ecliptic_position);
        println!("    Moon:");
        let moon_ecliptic_position = calculations::moon_position(julian_day - 1721013.5);
        println!("      Moon Position: {:?}", moon_ecliptic_position);
        
        
    }
}
