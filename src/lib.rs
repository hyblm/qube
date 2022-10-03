use std::{
    io::{Write, stdout},
    time::{Instant, Duration}};

use rand::{distributions::{Distribution, Standard}, Rng};

use crossterm::{queue, execute, cursor, Result,
    event::{read, poll, Event, KeyCode},
    style::{Print, Stylize}};

pub fn timer() -> Result<()> {
    let start_instant = Instant::now();
    let mut timer;
    let mut time;
    'timer: loop {
        timer = start_instant.elapsed();
        time = format!("{:.2}", timer.as_secs_f32());

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
    execute!(stdout(),
        cursor::Show,
        cursor::MoveDown(7),
    )?;
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
    let mut stdout = stdout();
    execute!(
        stdout,
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
            queue!(stdout, Print(row[col].magenta()))?;
        }
        queue!(stdout,
            cursor::MoveToColumn(1),
            cursor::MoveDown(1),
        )?;
    }
    stdout.flush()?;

    execute!(
        stdout,
        cursor::RestorePosition,
    )?;

    Ok(())
}

#[derive(PartialEq, Eq)]
pub enum Face {
    Front,
    Back,
    Right,
    Left,
    Up,
    Down,
}

impl Face {
    pub fn opposite(&self) -> Face {
        match self {
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
            Face::Up => Face::Down,
            Face::Down => Face::Up,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
             Face::Front => 'F',
             Face::Back => 'B',
             Face::Right => 'R',
             Face::Left => 'L',
             Face::Up => 'U',
             Face::Down => 'D',
        }
    }
}


impl Distribution<Face> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.gen_range(0..=6) {
            1 => Face::Front,
            2 => Face::Back,
            3 => Face::Right,
            4 => Face::Left,
            5 => Face::Up,
            _ => Face::Down,
        }
    }
}

/* impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
             Face::Front => write!("F"),
             Face::Back => write!('B'),
             Face::Right => write!('R'),
             Face::Left => write!('L'),
             Face::Up => write!('U'),
             Face::Down => write!('D'),
        }
    }
} */

//TODO: generate scrambles
pub enum Form {
    Clockwise,
    CounterClockwise,
    Double,
}
impl Form {
    pub fn to_str(&self) -> &str {
        match self {
           Form::Clockwise => "",
           Form::CounterClockwise => "'",
           Form::Double => "2",
        }
    }
}

impl Distribution<Form> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Form {
        match rng.gen_range(0..=3) {
            1 => Form::Clockwise,
            2 => Form::CounterClockwise,
            _ => Form::Double,
        }
    }
}


pub struct Move {pub face: Face, pub form: Form}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face.to_char(), self.form.to_str())
    }
}

pub fn generate_scramble() -> Vec<Move> {
    let mut scramble = Vec::new();

    let mut face = rand::random::<Face>();
    scramble.push(Move{face, form: rand::random::<Form>()});
    

    for i in 1..20 {

        loop {
            face = rand::random::<Face>();
            if face != scramble[i-1].face.opposite() && face != scramble[i-1].face {
                break;
            }
        }
        
        scramble.push(Move{ face, form: rand::random::<Form>()});

    }
    scramble
}
