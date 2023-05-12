use std::fmt::{Display, Formatter};
use colorize::AnsiColor;
use rand::Rng;

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Yellow,
    Red,
    Blue,
    Green,
    Violet,
    Orange,
}
impl Color {
    fn random() -> Color {
        match rand::thread_rng().gen_range(0..6) {
            0 => Color::Yellow,
            1 => Color::Red,
            2 => Color::Blue,
            3 => Color::Green,
            4 => Color::Violet,
            _ => Color::Orange,
        }
    }
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'y' => Color::Yellow,
            'r' => Color::Red,
            'b' => Color::Blue,
            'g' => Color::Green,
            'v' => Color::Violet,
            _ => Color::Orange,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "{}", "Y".yellowb()),
            Color::Red => write!(f, "{}", "R".redb()),
            Color::Blue => write!(f, "{}", "B".blueb()),
            Color::Green => write!(f, "{}", "G".greenb()),
            Color::Violet=> write!(f, "{}", "V".red()),
            Color::Orange=> write!(f, "{}", "O".red()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Evaluation {
    pub num_just_color: usize,
    pub num_position: usize,
}

#[derive(Debug)]
pub struct Combination(pub [Color;4]);
impl Combination {
    pub fn parse(s: &str) -> Option<Combination> {
        if s.len() != 5 { return None; }

        let chars: Vec<char> = s.chars().collect();
        Some(Combination([
            Color::from(chars[0]),
            Color::from(chars[1]),
            Color::from(chars[2]),
            Color::from(chars[3]),
        ]))
    }

    pub fn random() -> Combination {
        Combination([
            Color::random(),
            Color::random(),
            Color::random(),
            Color::random(),
        ])
    }
}

impl Display for Combination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0[0])?;
        write!(f, "{}", self.0[1])?;
        write!(f, "{}", self.0[2])?;
        write!(f, "{}", self.0[3])
    }
}

pub fn evaluate(c1: &Combination, c2: &Combination) -> Evaluation {
    let mut result = Evaluation::default();

    for ind_1 in 0..3 {
        if c1.0[ind_1] == c2.0[ind_1] {
            result.num_position += 1;
        }
        else if c2.0.contains(&c1.0[ind_1]) {
            result.num_just_color += 1;
        }
    }

    result
}

