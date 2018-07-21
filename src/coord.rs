use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(pub i8, pub i8);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn to_coord(&self) -> Coord {
        match self {
            Dir::Left => Coord(-1, 0),
            Dir::Right => Coord(1, 0),
            Dir::Down => Coord(0, 1),
            Dir::Up => Coord(0, -1),
        }
    }
}
