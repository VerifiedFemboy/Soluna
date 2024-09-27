
use geolocation::Locator;
use ratatui::{layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}, DefaultTerminal};
use std::io::Result;

pub struct App {
    terminal: DefaultTerminal,
    current_ip: String,
    geolocation: Locator,
}

impl App {
    pub fn new (terminal: DefaultTerminal, current_ip: String, geolocation: Locator) -> App {
        App {
            terminal,
            current_ip,
            geolocation,
        }
    }

    pub fn run_app(&mut self) -> Result<()> {
        ratatui::init();
        loop {
            self.render()?;
        }
    }

    fn render(&mut self) -> Result<()> {
        let terminal = &mut self.terminal;

        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(frame.area());

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            frame.render_widget(block, chunks[1]);
        });
        Ok(())
    }
}