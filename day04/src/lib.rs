use aoc_util::CardinalDirection;
use std::io::BufRead;

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut read = usize::MAX;
    let mut grid = Vec::new();

    while read > 0 {
        let mut line = Vec::new();
        read = reader.read_until(b'\n', &mut line).unwrap();
        grid.push(line);
    }

    let mut count_p1 = 0;
    let mut count_p2 = 0;
    for (vert, line) in grid.iter().enumerate() {
        for hor in 0..line.len() {
            if line[hor] == b'X' {
                count_p1 += find_matches_p1(&grid, hor, vert);
            } else if line[hor] == b'A' {
                count_p2 += find_matches_p2(&grid, hor, vert);
            }
        }
    }

    (count_p1, count_p2)
}

#[derive(Clone, Copy, Debug)]
pub struct SpiralArm {
    direction: SpirallingOutOfControl,
    hor: i32,
    vert: i32,
}
#[derive(Clone, Copy, Debug)]
enum SpirallingOutOfControl {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl SpiralArm {
    fn all_cardinalities() -> Vec<SpiralArm> {
        use SpirallingOutOfControl::*;
        Vec::from([
            Self::n(North),
            Self::n(NorthEast),
            Self::n(East),
            Self::n(SouthEast),
            Self::n(South),
            Self::n(SouthWest),
            Self::n(West),
            Self::n(NorthWest),
        ])
    }

    fn n(direction: SpirallingOutOfControl) -> SpiralArm {
        SpiralArm {
            direction,
            hor: 0,
            vert: 0,
        }
    }
    fn further_out(&mut self) {
        use SpirallingOutOfControl::*;
        match self.direction {
            North => {
                self.vert -= 1;
            }
            NorthEast => {
                self.hor += 1;
                self.vert -= 1;
            }
            East => {
                self.hor += 1;
            }
            SouthEast => {
                self.hor += 1;
                self.vert += 1;
            }
            South => {
                self.vert += 1;
            }
            SouthWest => {
                self.hor -= 1;
                self.vert += 1;
            }
            West => {
                self.hor -= 1;
            }
            NorthWest => {
                self.hor -= 1;
                self.vert -= 1;
            }
        }
    }

    fn in_bounds_and_proper(
        &self,
        expected: u8,
        hor: usize,
        vert: usize,
        grid: &Vec<Vec<u8>>,
    ) -> bool {
        if vert as i32 + self.vert < 0 {
            return false;
        }
        let vert_pos = (vert as i32 + self.vert) as usize;
        if let Some(line) = grid.get(vert_pos) {
            if hor as i32 + self.hor < 0 {
                return false;
            }
            let hor_pos = (hor as i32 + self.hor) as usize;
            if let Some(val) = line.get(hor_pos) {
                *val == expected
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn find_matches_p1(grid: &Vec<Vec<u8>>, hor: usize, vert: usize) -> usize {
    let mut direction_found = 0;
    'outer: for mut direction in SpiralArm::all_cardinalities() {
        for val in [b'M', b'A', b'S'] {
            direction.further_out();
            if !direction.in_bounds_and_proper(val, hor, vert, grid) {
                continue 'outer;
            }
        }
        direction_found += 1;
    }

    direction_found
}

fn get_in_bounds<T>(
    grid: &Vec<Vec<T>>,
    hor: usize,
    vert: usize,
    direction: CardinalDirection,
    steps: i32,
) -> Option<&T>
where
    T: Clone,
{
    let (hor_vc, ver_vc) = direction.direction_vec();
    let ver_idx = vert as i32 + ver_vc * steps;
    if ver_idx < 0 {
        return None;
    }
    let ver_idx = ver_idx as usize;
    if ver_idx >= grid.len() {
        return None;
    }

    let line = &grid[ver_idx];

    let hor_idx = hor as i32 + hor_vc * steps;
    if hor_idx < 0 {
        return None;
    }
    let hor_idx = hor_idx as usize;
    line.get(hor_idx)
}

fn find_matches_p2(grid: &Vec<Vec<u8>>, hor: usize, vert: usize) -> usize {
    use CardinalDirection::*;
    // valid 'slash'
    let ne_val = get_in_bounds(grid, hor, vert, NorthEast, 1);
    let nw_val = get_in_bounds(grid, hor, vert, NorthWest, 1);
    let se_val = get_in_bounds(grid, hor, vert, SouthEast, 1);
    let sw_val = get_in_bounds(grid, hor, vert, SouthWest, 1);

    let valid_slash = match (sw_val, ne_val) {
        (Some(b'M'), Some(b'S')) => true,
        (Some(b'S'), Some(b'M')) => true,
        _ => false,
    };

    let valid_downslash = match (nw_val, se_val) {
        (Some(b'M'), Some(b'S')) => true,
        (Some(b'S'), Some(b'M')) => true,
        _ => false,
    };

    if valid_slash && valid_downslash {
        1
    } else {
        0
    }
}
