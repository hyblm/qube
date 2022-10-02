use std::{
    io::{stdout, Write},
    time::{Instant, Duration}};

use crossterm::{ExecutableCommand, execute, cursor, Result,
    event::{read, poll, Event, KeyCode}};

pub fn timer() -> Result<()> {
    let start_instant = Instant::now();
    let mut timer;
    let mut time;
    'timer: loop {
        timer = start_instant.elapsed();
        time = format!("{:.2}", timer.as_secs_f32());

        // print!("{}", time);
        // stdout().execute(cursor::MoveToColumn(1))?;
        // println!();

        render_time(time)?;

        if poll(Duration::from_millis(10)).unwrap() {
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break 'timer,
                    _ => (),
                },
                _ => (),
            };
        };
    }
    Ok(())
}

const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

fn render_time(time: String) -> Result<()> {
    execute!(
        stdout(),
        cursor::Hide,
        cursor::MoveToColumn(1),
        cursor::SavePosition,
    )?;

    for row in &DIGITS {
        for c in time.chars() {
            let col = match c {
                '0'..='9' => c as usize - '0' as usize,
                ':' => 10,
                _ => 10,
            };
            print!("{} ", row[col]);
        }
        println!();
        stdout().execute(cursor::MoveToColumn(1))?;
    }

    execute!(
        stdout(),
        cursor::Show,
        cursor::RestorePosition,
    )?;

    Ok(())
}
