use std::{borrow::Cow, fmt::Display};

use crossterm::{event, style::Color};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Select,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NOR"),
            Mode::Insert => write!(f, "INS"),
            Mode::Select => write!(f, "SEL"),
            Mode::Command => write!(f, "CMD"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub default_bg: Color,
    pub status_bar_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            default_bg: Color::Black,
            status_bar_bg: Color::Rgb {
                r: 0x14,
                g: 0x14,
                b: 0x16,
            },
        }
    }
}

pub struct Cursor {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Editor {
    /// Current editing mode.
    pub mode: Mode,
    /// The currently applied editor theme.
    pub theme: Theme,

    /// It can be the path to a buffer or the name of a Pane.
    pub status_msg: Option<Cow<'static, str>>,

    pub cmd_buf: String,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            mode: Mode::Normal,
            theme: Default::default(),
            status_msg: None,
            cmd_buf: String::new(),
        }
    }

    pub fn handle_key_event(&mut self, event: event::KeyEvent) {
        match self.mode {
            Mode::Normal => {
                self.handle_key_event_normal(event);
            }
            Mode::Insert => {}
            Mode::Select => {}
            Mode::Command => {
                self.handle_key_event_command(event);
            }
        }
    }

    pub(crate) fn handle_key_event_normal(&mut self, event: event::KeyEvent) {
        match event {
            event::KeyEvent {
                code: event::KeyCode::Char(':'),
                ..
            } => {
                self.cmd_buf.clear();
                self.mode = Mode::Command;
            }
            _ => {}
        }
    }

    pub(crate) fn handle_key_event_command(&mut self, event: event::KeyEvent) {
        match event {
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => {
                self.cmd_buf.push(c);
            }
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => self.mode = Mode::Normal,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => {
                self.cmd_buf.pop();
            }
            _ => {}
        }
    }
}
