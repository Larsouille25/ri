use std::{error::Error, io::Stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        self, BeginSynchronizedUpdate, ClearType, DisableLineWrap, EnableLineWrap,
        EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

pub type RiResult<T = ()> = Result<T, Box<dyn Error>>;

pub struct RiApp {
    pub rows: u16,
    pub cols: u16,
    pub stdout: Stdout,
    pub quit: bool,
}

impl RiApp {
    pub fn new(stdout: Stdout) -> RiApp {
        RiApp {
            rows: 0,
            cols: 0,
            stdout,
            quit: false,
        }
    }

    pub fn run(&mut self) -> RiResult {
        execute!(self.stdout, Hide, EnterAlternateScreen, DisableLineWrap)?;

        terminal::enable_raw_mode()?;

        while !self.quit {
            self.poll_events()?;
            self.refresh()?;
            sleep(Duration::from_secs_f32(1.0 / 60.0));
        }

        terminal::disable_raw_mode()?;

        execute!(self.stdout, EnableLineWrap, LeaveAlternateScreen, Show)?;
        Ok(())
    }

    pub fn refresh(&mut self) -> RiResult {
        (self.cols, self.rows) = terminal::size()?;
        execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            BeginSynchronizedUpdate
        )?;

        let msg = "Ri - modern editor";
        let quit_msg = "Press CTRL + c to quit.";
        execute!(
            self.stdout,
            MoveTo(self.cols / 2 - msg.len() as u16 / 2, self.rows / 2),
            SetForegroundColor(Color::Rgb {
                r: 87,
                g: 130,
                b: 247
            }),
            Print(msg),
            MoveTo(self.cols / 2 - quit_msg.len() as u16 / 2, self.rows / 2 + 1),
            SetForegroundColor(Color::White),
            Print(quit_msg)
        )?;

        execute!(self.stdout, EndSynchronizedUpdate)?;
        Ok(())
    }

    pub fn poll_events(&mut self) -> RiResult {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.quit = true;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
