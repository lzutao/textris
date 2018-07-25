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

impl Default for Coord {
    fn default() -> Self {
        Coord(0, 0)
    }
}

pub type Dirs = [Dir; 4];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn all() -> Dirs {
        [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
    }

    pub fn to_coord(&self) -> Coord {
        match self {
            Dir::Up => Coord(0, -1),
            Dir::Right => Coord(1, 0),
            Dir::Down => Coord(0, 1),
            Dir::Left => Coord(-1, 0),
        }
    }

    pub fn next_dir(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn opponent(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Dir::Up
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RotateDir {
    Clockwise,
    AntiClockwise,
}

impl RotateDir {
    pub fn rotate(&self, dir: Dir) -> Dir {
        let dir = dir.next_dir();
        match self {
            RotateDir::Clockwise => dir,
            RotateDir::AntiClockwise => dir.opponent(),
        }
    }
}
