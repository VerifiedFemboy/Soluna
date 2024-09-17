# Solar Calculations in Rust

This Rust project calculates solar declination and hour angle based on the current UTC time. The program continuously updates these values every 500 milliseconds and prints them to the console.

## Project Structure

- `main.rs`: The main entry point of the program.
- `calculations.rs`: Contains the functions for calculating Julian Day, solar declination, and solar hour angle.

## Dependencies

This project uses the following dependencies:
- `chrono`: For handling date and time.

## How It Works

### Main Loop

The `main` function contains an infinite loop that:
1. Clears the console.
2. Sleeps for 500 milliseconds.
3. Gets the current UTC time.
4. Calculates the Julian Day.
5. Calculates the current time in hours, minutes, and seconds.
6. Calculates the solar declination and hour angle.
7. Prints the current time, Julian Day, declination, and hour angle to the console.

```rust
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