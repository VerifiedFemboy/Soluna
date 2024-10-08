
use chrono::{Datelike, Timelike};
use crossterm::event::{self, Event, KeyCode};
use geolocation::Locator;
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Stylize}, widgets::{Block, Borders, Paragraph}, DefaultTerminal};
use std::io::Result;

use crate::{calculations, location};

pub struct App {
    terminal: DefaultTerminal,
    current_ip: String,
    geolocation: Locator,
    exit: bool,
}

impl App {
    pub fn new (terminal: DefaultTerminal, current_ip: String, geolocation: Locator) -> App {
        App {
            terminal,
            current_ip,
            geolocation,
            exit: false,
        }
    }

    pub fn run_app(&mut self) -> Result<()> {
        Ok(while !self.exit {
            self.render()?;
            self.handle_input()?;
        })
    }
    
    fn render(&mut self) -> Result<()> {
        let terminal = &mut self.terminal;
        
        let current_time: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let time = current_time.time().hour() as f64 + current_time.minute() as f64 / 60.0 + current_time.second() as f64 / 3600.0;
        let current_time_formated = current_time.format("%H:%M:%S | %m-%d-%Y").to_string();

        let latidude = self.geolocation.latitude.parse::<f64>().unwrap();
        let longtidue = self.geolocation.longitude.parse::<f64>().unwrap();

        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(10),
                        Constraint::Min(10),
                        Constraint::Min(10),
                    ]
                    .as_ref(),
                )
                .split(frame.area());
            
            let block = Block::default()
                            .title("Location Information")
                            .borders(Borders::ALL).fg(Color::Green);

            let country_emote = location::country_emote(self.geolocation.country.clone());
            let location_paragraph = ratatui::widgets::Paragraph::new(
                format!("IP: {}\n🕜Current Time: {}\n🌐Location ({}, {}): \n  🏙️City: {}\n  {country_emote}Country: {}\n  Region: {}\n  Timezone: {}",
                 self.current_ip,
                 current_time_formated,
                 longtidue,
                 latidude,
                 self.geolocation.city, 
                 self.geolocation.country, 
                 self.geolocation.region, self.geolocation.timezone))
                        .block(block).style(Color::White);

            frame.render_widget(location_paragraph, chunks[0]);



            let solar_block = Block::default().title("☀️ Solar Information")
            .borders(Borders::ALL).fg(Color::Yellow);

            let current_day_of_year = current_time.ordinal() as f64;
            let julian_day = calculations::calculate_julian_day(current_time);
            let hour_angle = calculations::solar_hour_angle(time, longtidue);

            let solar_paragraph = Paragraph::new(
                format!("Current Day Of Year: {}\nJulian Day: {}\nDeclination: {}\nHour Angle: {}\nEcliptic Position: {}", 
                current_day_of_year, julian_day, calculations::solar_declination(current_day_of_year), hour_angle, calculations::solar_ecliptic_position(julian_day - 1721013.5)))
                .style(Color::White)
                .block(solar_block);
            
            frame.render_widget(solar_paragraph, chunks[1]);


            let moon_block = Block::default().title("🌕 Moon Information")
            .borders(Borders::ALL).fg(Color::Rgb(71, 71, 71));

            let moon_position = calculations::moon_position(julian_day - 1721013.5);

            let moon_paragraph = Paragraph::new(
                format!("Position: {:?}", moon_position))
                .style(Color::White)
                .block(moon_block).style(Color::White);

            frame.render_widget(moon_paragraph, chunks[2]);
        });
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        self.exit = true;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}