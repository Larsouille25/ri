use std::{io::stdout, process::ExitCode};

use ri::RiApp;

fn main() -> ExitCode {
    let mut app = RiApp::new(stdout());

    match app.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
