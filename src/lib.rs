use std::error::Error;
use std::io::{Stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::QueueableCommand;
use crossterm::{
    cursor, event, execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};
use editor::{Editor, Mode};

pub type RiResult<T = ()> = Result<T, Box<dyn Error>>;

pub mod editor;

pub struct RiApp {
    /// Number of rows of the terminal
    pub rows: u16,
    /// Number of columns of the terminal
    pub cols: u16,

    pub stdout: Stdout,
    /// Should we quit Ri in the next loop iteration?
    pub quit: bool,

    /// The editor
    pub editor: Editor,
}

impl RiApp {
    pub fn new(stdout: Stdout) -> RiApp {
        RiApp {
            rows: 0,
            cols: 0,
            stdout,
            quit: false,
            editor: Editor::new(),
        }
    }

    pub fn run(&mut self) -> RiResult {
        execute!(
            self.stdout,
            cursor::Hide,
            terminal::EnterAlternateScreen,
            terminal::DisableLineWrap
        )?;

        terminal::enable_raw_mode()?;

        while !self.quit {
            self.poll_events()?;
            self.refresh()?;
            // TODO: mesure the duration when the loop finished one execution
            // and only wait the appropriate amound of time.
            sleep(Duration::from_secs_f32(1.0 / 60.0));
        }

        terminal::disable_raw_mode()?;

        execute!(
            self.stdout,
            terminal::EnableLineWrap,
            terminal::LeaveAlternateScreen,
            cursor::Show
        )?;
        Ok(())
    }

    pub fn refresh(&mut self) -> RiResult {
        (self.cols, self.rows) = terminal::size()?;
        queue!(
            self.stdout,
            terminal::Clear(ClearType::All),
            terminal::BeginSynchronizedUpdate
        )?;

        self.status_bar()?;
        let msg = "Ri - modern editor";
        let quit_msg = "Press CTRL + c to quit.";
        queue!(
            self.stdout,
            cursor::MoveTo(self.cols / 2 - msg.len() as u16 / 2, self.rows / 2),
            style::SetForegroundColor(Color::Rgb {
                r: 87,
                g: 130,
                b: 247
            }),
            style::Print(msg),
            cursor::MoveTo(self.cols / 2 - quit_msg.len() as u16 / 2, self.rows / 2 + 1),
            style::SetForegroundColor(Color::White),
            style::Print(quit_msg)
        )?;

        queue!(self.stdout, terminal::EndSynchronizedUpdate)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn poll_events(&mut self) -> RiResult {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                event::Event::Key(event::KeyEvent {
                    code: event::KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.quit = true;
                }
                Event::Key(event) => {
                    self.editor.handle_key_event(event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Method responsible for the rendering of the status bar.
    pub fn status_bar(&mut self) -> RiResult {
        queue!(
            self.stdout,
            // Render the bar with its background color
            cursor::MoveTo(0, self.rows - 2),
            style::SetBackgroundColor(self.editor.theme.status_bar_bg),
            style::Print(String::from_utf8(vec![b' '; self.cols as usize]).unwrap()),
            // Render the mode of the editor
            cursor::MoveTo(1, self.rows - 2),
            style::Print(self.editor.mode.clone()),
        )?;

        // Render the status message
        if let Some(msg) = self.editor.status_msg.clone() {
            queue!(self.stdout, cursor::MoveRight(3), style::Print(msg))?;
        }

        // Reset to the default bg color
        self.stdout
            .queue(style::SetBackgroundColor(self.editor.theme.default_bg))?;

        // Render the command if in command mode.
        if self.editor.mode == Mode::Command {
            queue!(
                self.stdout,
                cursor::MoveTo(0, self.rows - 1),
                style::Print(":"),
                style::Print(&self.editor.cmd_buf),
                style::Print("█"),
            )?;
        }
        Ok(())
    }
}
