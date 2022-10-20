use qube::*;

use std::io::stdout;

use crossterm::{ cursor,
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType}, execute};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut times: Vec<f32> = vec![0.0];

    'main: loop {
        init_screen(&mut stdout)?;

        if let Event::Key(key) = read()? { match key.code {
            KeyCode::Char('q') => {
                render_time(&mut stdout, format!("{:.2}",average(&times)))?;
                execute!(
                    stdout,
                    cursor::MoveToNextLine(7),
                    Clear(ClearType::FromCursorDown),
                )?;
                print!("Your average for this session");
                break 'main
            },
            KeyCode::Char(' ') => timer(&mut stdout, &mut times)?,
            _ => (),
        } };
    }
    execute!(stdout,cursor::Show)?;
    disable_raw_mode()
}
