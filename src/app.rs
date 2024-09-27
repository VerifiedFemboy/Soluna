
use chrono::Datelike;
use crossterm::event::{self, Event, KeyCode};
use geolocation::Locator;
use ratatui::{layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph}, DefaultTerminal};
use std::io::Result;

use crate::calculations;

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
        let current_time_formated = current_time.format("%H:%M:%S | %m-%d-%Y").to_string();
        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(60),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(frame.area());
            
            let block = Block::default()
                            .title("Location Information")
                            .borders(Borders::ALL);

            let location_paragraph = ratatui::widgets::Paragraph::new(
                format!("IP: {}\nCurrent Time: {}\nLocation {}, {}: \nCity: {}\nCountry: {}\nRegion: {}\nTimezone: {}",
                 self.current_ip,
                 current_time_formated,
                 self.geolocation.latitude,
                 self.geolocation.longitude,
                 self.geolocation.city, 
                 self.geolocation.country, 
                 self.geolocation.region, self.geolocation.timezone))
                        .block(block);

            frame.render_widget(location_paragraph, chunks[0]);



            let solar_block = Block::default().title("Solar Information")
            .borders(Borders::ALL);

            let current_day_of_year = current_time.ordinal() as f64;

            let solar_paragraph = Paragraph::new(
                format!("Declination: {}", calculations::solar_declination(current_day_of_year)))
                .block(solar_block);
            
            frame.render_widget(solar_paragraph, chunks[1]);
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