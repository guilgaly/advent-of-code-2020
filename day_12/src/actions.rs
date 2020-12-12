use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Position {
    pub north: i64,
    pub east: i64,
}

impl Position {
    pub fn move_towards(&self, orientation: Orientation, distance: i64) -> Position {
        let (north, east) = match orientation {
            Orientation::N => (self.north + distance, self.east),
            Orientation::S => (self.north - distance, self.east),
            Orientation::E => (self.north, self.east + distance),
            Orientation::W => (self.north, self.east - distance),
        };
        Position { north, east }
    }

    pub fn move_by(&self, vect: Position) -> Position {
        Position {
            north: self.north + vect.north,
            east: self.east + vect.east,
        }
    }

    pub fn scaled(&self, multiplier: i64) -> Position {
        Position {
            north: self.north * multiplier,
            east: self.east * multiplier,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Orientation {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    L,
    R,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Move(Orientation, i64),
    Rotate(Direction, i64),
    F(i64),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let action = chars.next().ok_or("Empty string")?;
        let value = chars
            .as_str()
            .parse::<i64>()
            .map_err(|_| format!("Not a valide number in {}", s))?;
        match action {
            'N' => Ok(Action::Move(Orientation::N, value)),
            'S' => Ok(Action::Move(Orientation::S, value)),
            'E' => Ok(Action::Move(Orientation::E, value)),
            'W' => Ok(Action::Move(Orientation::W, value)),
            'L' => Ok(Action::Rotate(Direction::L, value / 90)),
            'R' => Ok(Action::Rotate(Direction::R, value / 90)),
            'F' => Ok(Action::F(value)),
            _ => Err(format!("Not a valid action in {}", s)),
        }
    }
}
