use core::fmt;
use std::io::Stdout;

use keys::{Key, KeyMap};
use ratatui::{
    prelude::CrosstermBackend,
    style::Stylize,
    text::{Span, Text},
    widgets::{List, ListDirection, Paragraph, Widget},
    Terminal,
};

pub mod keys {
    use core::{fmt, slice};
    use crossterm::event::KeyCode;
    use ratatui::{layout::Rect, widgets::Widget};
    use std::collections::HashMap;

    pub trait KeyMap<T> {
        fn register(&mut self, key: Key<T>);
        fn make_iter(&self) -> slice::Iter<'_, Key<T>>;
    }

    impl<T> KeyMap<T> for Vec<Key<T>> {
        fn register(&mut self, key: Key<T>) {
            self.push(key);
        }

        fn make_iter(&self) -> slice::Iter<'_, Key<T>> {
            self.iter()
        }
    }

    // pub type KeyMap<T> = Vec<Key<T>>;
    pub struct Key<T> {
        code: T,
        short: String,
    }

    impl<T> Key<T> {
        pub fn new(code: T, short: String) -> Self {
            Self { code, short }
        }
    }

    impl<T> fmt::Display for Key<T>
    where
        T: fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let format = format!("[{}] -- {}", self.code, self.short);
            write!(f, "{}", format)
        }
    }
}

struct App {}

impl App {
    pub fn key_map_widget<T: fmt::Display>(&self, key_map: impl KeyMap<T>) -> impl Widget {
        let items: Vec<Span> = key_map
            .make_iter()
            .map(|key| key.to_string())
            .map(Span::from)
            .collect();

        List::new(items).white().on_black()
    }

    pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> std::io::Result<()> {
        Ok(())
    }
}
