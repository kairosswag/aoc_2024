use aoc_util::CardinalDirection;
use std::io::BufRead;

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut read = usize::MAX;
    let mut grid = Vec::with_capacity(256);

    while read > 0 {
        let mut line = Vec::with_capacity(256);
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

fn find_matches_p1(grid: &Vec<Vec<u8>>, hor: usize, vert: usize) -> usize {
    let mut direction_found = 0;
    for direction in CardinalDirection::iter() {
        if let Some(b'S') = get_in_bounds(grid, hor, vert, *direction, 3) {
            if b'A' == get_confident(grid, hor, vert, *direction, 2) {
                if b'M' == get_confident(grid, hor, vert, *direction, 1) {
                    direction_found += 1;
                }
            }
        }
    }

    direction_found
}

fn get_confident(
    grid: &Vec<Vec<u8>>,
    hor: usize,
    vert: usize,
    direction: CardinalDirection,
    steps: i32,
) -> u8 {
    let (hor_vc, ver_vc) = direction.direction_vec();
    let ver_idx = vert as i32 + ver_vc * steps;
    let line = &grid[ver_idx as usize];
    let hor_idx = hor as i32 + hor_vc * steps;
    line[hor_idx as usize]
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
    if hor_idx >= line.len() {
        return None;
    }
    Some(&line[hor_idx])
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
