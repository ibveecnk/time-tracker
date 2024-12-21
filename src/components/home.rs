use std::num::Wrapping;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    progress: Wrapping<u8>,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
                self.progress += Wrapping(1);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                ratatui::layout::Constraint::Percentage(10),
                ratatui::layout::Constraint::Percentage(70),
                ratatui::layout::Constraint::Percentage(20),
            ])
            .split(frame.area());

        frame.render_widget(Paragraph::new("Home"), layout[0]);
        frame.render_widget(
            Gauge::default().ratio(self.progress.0 as f64 / u8::MAX as f64),
            layout[1],
        );
        frame.render_widget(Paragraph::new("Timer"), layout[2]);
        Ok(())
    }
}
