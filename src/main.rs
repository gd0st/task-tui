use std::{
    cmp::{max, min},
    env,
    fs::OpenOptions,
    io::{self, stdout, Stdout},
    rc::Rc,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    self,
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::{Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{ListItem, ListState, Paragraph, Widget},
    Terminal,
};
use serde::{Deserialize, Serialize};
use tasklist::keys::{self, Key};
fn main() -> anyhow::Result<()> {
    // Got the main list ui figured out. Need to be able to load from a premade list.

    // todo!

    // Find and load the list

    let Ok(home_var) = env::var("HOME") else {
        eprintln!("HOME variable not set");
        return Ok(());
    };

    let filepath = format!("{}/.tasks.json", home_var);

    let reader = OpenOptions::new().read(true).open(&filepath);

    let mut tasks: Vec<TaskItem> = {
        if let Ok(reader) = reader {
            serde_json::from_reader(reader).unwrap_or(vec![])
        } else {
            vec![]
        }
    };
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());

    let mut terminal = ratatui::Terminal::new(backend)?;
    let app_result = run(&mut terminal, &mut tasks)?;
    disable_raw_mode()?;

    if let Ok(writer) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&filepath)
    {
        serde_json::to_writer(writer, &tasks)?;
    } else {
        eprintln!("error while writing to {}", &filepath);
    }

    // Save the list.
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

#[derive(PartialEq)]
enum Mode {
    Insert,
    Browse,
}

impl Mode {
    pub fn get_key_map(&self) -> Vec<Key<String>> {
        match self {
            Self::Browse => {
                let keys = [keys::Key::new(
                    "i".to_string(),
                    "insert new task mode".to_string(),
                )];
                keys.into_iter().collect()
            }
            _ => vec![],
        }
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    tasks: &mut Vec<TaskItem>,
) -> io::Result<()> {
    // let mut tasks: Vec<TaskItem> = vec![];
    let mut state = ListState::default();
    let mut insert_mode = Mode::Browse;
    let mut insert_buffer = String::new();
    let mut index = 0;
    loop {
        if let Some(event) = event_poll(64)? {
            if insert_mode == Mode::Insert {
                match event {
                    KeyCode::Enter => {
                        insert_mode = Mode::Browse;
                        tasks.add(insert_buffer.as_str());
                        insert_buffer.clear();
                    }
                    KeyCode::Char(c) => insert_buffer.push(c),
                    KeyCode::Backspace => {
                        insert_buffer.pop();
                    }
                    _ => {}
                }
            } else {
                match event {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('c') => {
                        tasks.close(index);
                    }
                    KeyCode::Char('k') => {
                        if index != 0 {
                            index -= 1;
                        }
                    }
                    KeyCode::Char('i') => {
                        insert_mode = Mode::Insert;
                    }
                    KeyCode::Char('0') => tasks.clear(),
                    KeyCode::Char('j') => {
                        index = min(tasks.len() - 1, index + 1);
                    }
                    _ => {}
                }
            }
        }

        state.select(Some(index));

        let items = tasks.iter().map(|item| {
            let text = Span::from(item.name.to_string());
            if item.closed {
                text.crossed_out()
            } else {
                text
            }
        });

        let tui_list = ratatui::widgets::List::new(items)
            .white()
            .on_black()
            .highlight_symbol(">> ")
            .direction(ratatui::widgets::ListDirection::TopToBottom);
        terminal.draw(|frame| {
            let area = if insert_mode == Mode::Insert {
                let constraints = [(3, 4), (1, 4)];
                let areas = Layout::default()
                    .constraints(Constraint::from_ratios(constraints))
                    .direction(ratatui::layout::Direction::Vertical)
                    .split(frame.area());

                let insert_text = format!("> {}", insert_buffer.clone());
                frame.render_widget(Text::from(insert_text).white().on_black(), areas[1]);
                areas[0]
            } else {
                frame.area()
            };
            frame.render_stateful_widget(tui_list, area, &mut state);
        })?;
    }
}

#[derive(Serialize, Deserialize)]
struct TaskItem {
    name: String,
    closed: bool,
}

impl From<&str> for TaskItem {
    fn from(value: &str) -> Self {
        TaskItem {
            name: value.to_string(),
            closed: false,
        }
    }
}

trait TaskList
where
    Self: Sized,
{
    fn add(&mut self, name: &str);
    fn close(&mut self, index: usize);
}

impl TaskList for Vec<TaskItem> {
    fn add(&mut self, name: &str) {
        self.push(TaskItem::from(name))
    }

    fn close(&mut self, index: usize) {
        if let Some(item) = self.get_mut(index) {
            item.closed = true
        }
    }
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
