use std::{
    io::Write,
    time::{Duration, Instant},
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crossterm::{
    cursor,
    event::{poll, read, Event},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    Result,
};

pub fn timer(stdout: &mut std::io::Stdout, times: &mut Vec<f32>) -> Result<()> {
    let start_instant = Instant::now();
    let mut time;
    'timer: loop {
        time = start_instant.elapsed().as_secs_f32();
        render_time(stdout, format!("{:.2}", time))?;

        if poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(_key) = read()? {
                times.push(time);
                break 'timer;
            };
        };
    }
    execute!(stdout, cursor::Show,)?;
    Ok(())
}

#[rustfmt::skip]
const DIGITS: [[&str; 11]; 7] = [
    [ "┏━┓ ", "  ╻  ", " ┏━┓ ", " ┏━┓ ", " ╻ ╻ ", " ┏━┓ ", " ┏   ", " ┏━┓ ", " ┏━┓ ", " ┏━┓ ", "   ", ],
    [ "┃ ┃ ", "  ┃  ", "   ┃ ", "   ┃ ", " ┃ ┃ ", " ┃   ", " ┃   ", "   ┃ ", " ┃ ┃ ", " ┃ ┃ ", " ╻ ", ],
    [ "┃ ┃ ", "  ┃  ", "   ┃ ", "   ┃ ", " ┃ ┃ ", " ┃   ", " ┃   ", "   ┃ ", " ┃ ┃ ", " ┃ ┃ ", "   ", ],
    [ "┃ ┃ ", "  ┃  ", " ┏━┛ ", " ┣━┫ ", " ┗━┫ ", " ┗━┓ ", " ┣━┓ ", "   ┃ ", " ┣━┫ ", " ┗━┫ ", "   ", ],
    [ "┃ ┃ ", "  ┃  ", " ┃   ", "   ┃ ", "   ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", "   ", ],
    [ "┃ ┃ ", "  ┃  ", " ┃   ", "   ┃ ", "   ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ╹ ", ],
    [ "┗━┛ ", "  ╹  ", " ┗━━ ", " ┗━┛ ", "   ╹ ", " ┗━┛ ", " ┗━┛ ", "   ╹ ", " ┗━┛ ", " ┗━┛ ", "   ", ],
];

pub fn render_time(stdout: &mut std::io::Stdout, time: String) -> Result<()> {
    queue!(
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
        queue!(stdout, cursor::MoveToNextLine(1),)?;
    }
    queue!(stdout, cursor::RestorePosition)?;

    stdout.flush()?;

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
    pub fn to_str(&self) -> &str {
        match self {
            Face::Front => "F",
            Face::Back => "B",
            Face::Right => "R",
            Face::Left => "L",
            Face::Up => "U",
            Face::Down => "D",
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

pub enum Mod {
    Clockwise,
    CounterClockwise,
    Double,
}
impl Mod {
    pub fn to_str(&self) -> &str {
        match self {
            Mod::Clockwise => "",
            Mod::CounterClockwise => "'",
            Mod::Double => "2",
        }
    }
}

impl Distribution<Mod> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mod {
        match rng.gen_range(0..=3) {
            1 => Mod::Clockwise,
            2 => Mod::CounterClockwise,
            _ => Mod::Double,
        }
    }
}

pub struct Move {
    pub face: Face,
    pub form: Mod,
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face.to_str(), self.form.to_str())
    }
}

pub fn generate_scramble() -> Vec<Move> {
    let mut scramble = Vec::new();
    scramble.push(Move {
        face: rand::random::<Face>(),
        form: rand::random::<Mod>(),
    });

    for i in 1..20 {
        loop {
            let face = rand::random::<Face>();
            if face != scramble[i - 1].face && face != scramble[i - 1].face.opposite() {
                scramble.push(Move {
                    face,
                    form: rand::random::<Mod>(),
                });
                break;
            }
        }
    }
    scramble
}

pub fn init_screen(stdout: &mut std::io::Stdout, time: &f32) -> Result<()> {
    queue!(
        stdout,
        Clear(ClearType::FromCursorDown),
        cursor::SavePosition,
    )?;

    render_time(stdout, format!("{:.2}", time))?;

    queue!(stdout, cursor::MoveDown(7))?;

    for turn in generate_scramble() {
        match turn.face {
            Face::Front => queue!(stdout, SetForegroundColor(Color::Green))?,
            Face::Back => queue!(stdout, SetForegroundColor(Color::Blue))?,
            Face::Right => queue!(stdout, SetForegroundColor(Color::Red))?,
            Face::Left => queue!(
                stdout,
                SetForegroundColor(Color::Rgb {
                    r: 200,
                    g: 130,
                    b: 50
                })
            )?,
            Face::Up => queue!(stdout, SetForegroundColor(Color::White))?,
            Face::Down => queue!(stdout, SetForegroundColor(Color::Yellow))?,
        }
        queue!(stdout, Print(format!("{} ", turn).bold()))?;
    }

    queue!(
        stdout,
        ResetColor,
        cursor::MoveToNextLine(1),
        Print("Press space to start the timer, Q to quit"),
        cursor::RestorePosition,
    )?;

    stdout.flush()?;

    Ok(())
}

pub fn average(times: &[f32]) -> f32 {
    times.iter().sum::<f32>() / (times.len() - 1) as f32
}
