use std::ops::Add;
use std::slice::Iter;
use CardinalDirection::*;
#[derive(Clone, Copy, Debug)]
pub enum CardinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl CardinalDirection {
    const ALL_VALUES: [CardinalDirection; 8] = [
        North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ];

    pub fn direction_vec(&self) -> (i32, i32) {
        use CardinalDirection::*;
        match self {
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }

    pub fn iter() -> Iter<'static, CardinalDirection> {
        Self::ALL_VALUES.iter()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub ver_idx: isize,
    pub hor_idx: isize,
}

impl Coordinate {
    pub fn new(ver_idx: isize, hor_idx: isize) -> Coordinate {
        Coordinate { ver_idx, hor_idx }
    }
    pub fn new_u(ver_idx: usize, hor_idx: usize) -> Coordinate {
        Coordinate { ver_idx: ver_idx as isize, hor_idx: hor_idx as isize }
    }


}

impl From<(isize, isize)> for Coordinate {
    fn from(value: (isize, isize)) -> Self {
        Coordinate {
            ver_idx: value.0,
            hor_idx: value.1,
        }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            ver_idx: self.ver_idx + rhs.ver_idx,
            hor_idx: self.hor_idx + rhs.hor_idx,
        }
    }
}
