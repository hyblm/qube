use std::{
    io::Write,
    time::{Instant, Duration}};

use rand::{distributions::{Distribution, Standard}, Rng};

use crossterm::{queue, execute, cursor, Result,
    event::{read, poll, Event},
    style::{Print, Stylize}};

pub fn timer(stdout: &mut std::io::Stdout, times: &mut Vec<f32>) -> Result<()> {
    let start_instant = Instant::now();
    let mut timer;
    let mut time;
    let mut time_string;
    'timer: loop {
        timer = start_instant.elapsed();
        time = timer.as_secs_f32();
        time_string = format!("{:.2}", time);

        render_time(stdout, time_string)?;

        if poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(_key) = read()? { 
                times.insert(0, time);
                break 'timer };
        };
    }
    execute!(stdout,
        cursor::Show,
        cursor::MoveDown(9),
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

pub fn render_time(stdout: &mut std::io::Stdout, time: String) -> Result<()> {
    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveToColumn(0),
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
            cursor::MoveToNextLine(1),
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

pub fn init_screen(stdout: &mut std::io::Stdout) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveToNextLine(1),
        cursor::SavePosition,
    )?;
    render_time(stdout, "0:00".to_string())?;

    execute!(stdout,
        cursor::MoveDown(7),
    )?;

    for turn in generate_scramble() {
        print!("{} ", turn);
    }
    execute!(stdout, cursor::MoveToNextLine(1))?;

    println!("Press space to start the timer, Q to quit");

    execute!(stdout, cursor::RestorePosition)?;

    Ok(())
}

pub fn average(times: &[f32]) -> f32 {
    times.iter().sum::<f32>() / (times.len() as f32 - 1.0)
}
