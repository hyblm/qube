use qube::*;

use std::time::Duration;

use crossterm::{
    event::{read, poll, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode}};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;

    let scramble = generate_scramble();
    for turn in scramble {
        print!("{}", turn);
    }

    println!("\nPress space to start the timer");

    'main: loop {
        if poll(Duration::from_millis(500)).unwrap() {
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break 'main,
                    KeyCode::Char(' ') => timer()?,
                    _ => (),
                },
                _ => (),
            };
        }
    }
    disable_raw_mode()

    //TODO: integrate with figlet fonts
}
