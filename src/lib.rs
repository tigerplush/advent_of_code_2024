use std::{num::ParseIntError, ops::Add};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
}

impl Position {
    pub const fn new(x: isize, y: isize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }

    pub const fn index(&self) -> usize {
        self.x as usize + self.y as usize * self.height
    }

    pub const fn in_bounds(&self) -> bool {
        self.x >= 0 && self.x < self.width as isize && self.y >= 0 && self.y < self.height as isize
    }

    pub const fn advance(&mut self) {
        self.x += 1;
        if self.x >= self.width as isize {
            self.x = 0;
            self.y += 1;
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let x = self.x as isize + rhs.x;
        let y = self.y as isize + rhs.y;
        Self {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::new(0, -1),
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(-1, 0),
];

#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub x: isize,
    pub y: isize,
}

impl Direction {
    pub const ONE: Direction = Direction::new(1, 1);
    pub const fn new(x: isize, y: isize) -> Self {
        Direction {
            x,
            y,
        }
    }
}