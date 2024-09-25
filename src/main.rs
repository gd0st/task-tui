use std::{
    cmp::{max, min},
    io::{self, stdout, Stdout},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    self,
    prelude::CrosstermBackend,
    style::{Modifier, Style, Stylize},
    widgets::{ListState, Paragraph, Widget},
    Terminal,
};
fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());

    let mut terminal = ratatui::Terminal::new(backend)?;
    let app_result = run(&mut terminal)?;
    disable_raw_mode()?;
    Ok(())
}

fn event_poll(interval: u64) -> std::io::Result<Option<KeyCode>> {
    let _ = event::poll(std::time::Duration::from_millis(interval))? || return Ok(None);

    if let Event::Key(event) = event::read()? {
        match event.kind {
            KeyEventKind::Press => Ok(Some(event.code)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut tasks: Vec<String> = vec![];
    let mut state = ListState::default();
    tasks.add("clean my room");
    let mut index = 0;
    loop {
        if let Some(event) = event_poll(64)? {
            match event {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('a') => {
                    tasks.add("Clean another room");
                }

                KeyCode::Char('k') => {
                    if index != 0 {
                        index -= 1;
                    }
                }
                KeyCode::Char('j') => {
                    index = min(tasks.len() - 1, index + 1);
                }
                _ => {}
            }
        }

        state.select(Some(index));
        let tui_list = ratatui::widgets::List::new(tasks.clone())
            .white()
            .on_black()
            .highlight_symbol(">>")
            .direction(ratatui::widgets::ListDirection::TopToBottom)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World!").white().on_blue();
            frame.render_stateful_widget(tui_list, frame.area(), &mut state);
        })?;
    }
}

trait TaskList
where
    Self: Sized,
{
    fn add(&mut self, name: &str);
    fn close(&mut self, index: usize);
}

impl TaskList for Vec<String> {
    fn add(&mut self, name: &str) {
        self.push(name.to_string());
    }

    fn close(&mut self, index: usize) {
        if index < self.len() {
            self.remove(index);
        }
    }
}
