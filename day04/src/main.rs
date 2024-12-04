use crate::SpirallingOutOfControl::*;
use std::io;
use std::io::BufRead;

pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

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
    let mut possible_directions = SpiralArm::all_cardinalities();
    for val in [b'M', b'A', b'S'] {
        let mut next_directions = Vec::new();
        for direction in &possible_directions {
            let mut direction = direction.clone();
            direction.further_out();
            if direction.in_bounds_and_proper(val, hor, vert, grid) {
                next_directions.push(direction);
            }
        }
        possible_directions = next_directions;
    }

    possible_directions.len()
}

fn find_matches_p2(grid: &Vec<Vec<u8>>, hor: usize, vert: usize) -> usize {
    let mut ne = SpiralArm::n(NorthEast);
    let mut nw = SpiralArm::n(NorthWest);
    let mut se = SpiralArm::n(SouthEast);
    let mut sw = SpiralArm::n(SouthWest);

    ne.further_out();
    nw.further_out();
    se.further_out();
    sw.further_out();

    // valid 'slash'
    let valid_slash = (sw.in_bounds_and_proper(b'M', hor, vert, grid)
        && ne.in_bounds_and_proper(b'S', hor, vert, grid))
        || (sw.in_bounds_and_proper(b'S', hor, vert, grid)
            && ne.in_bounds_and_proper(b'M', hor, vert, grid));
    let valid_downslash = (nw.in_bounds_and_proper(b'M', hor, vert, grid)
        && se.in_bounds_and_proper(b'S', hor, vert, grid))
        || (nw.in_bounds_and_proper(b'S', hor, vert, grid)
            && se.in_bounds_and_proper(b'M', hor, vert, grid));

    if valid_slash && valid_downslash {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests_p4 {
    use crate::run;

    #[test]
    fn test() {
        let test_iput = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!((18, 9), run(test_iput.as_bytes()));
    }
}
