use hashbrown::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::RangeInclusive;
use Direction::*;

type MapIndex = (usize, usize);

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_pos(&self, pos: MapIndex) -> MapIndex {
        match self {
            North => (pos.0 - 1, pos.1),
            East => (pos.0, pos.1 + 1),
            South => (pos.0 + 1, pos.1),
            West => (pos.0, pos.1 - 1),
        }
    }

    fn can_move(&self, pos: (usize, usize), max_vert: usize, max_hor: usize) -> bool {
        match self {
            North => pos.0 > 0,
            East => pos.1 < max_hor,
            South => pos.0 < max_vert,
            West => pos.1 > 0,
        }
    }

    fn next_direction(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn loops_at_intersect(&self, traversed: &Traversed) -> bool {
        match self {
            North => traversed.west_to_east,
            East => traversed.north_to_south,
            South => traversed.east_to_west,
            West => traversed.south_to_north,
        }
    }
}

#[derive(Debug)]
pub struct Traversed {
    east_to_west: bool,
    west_to_east: bool,
    north_to_south: bool,
    south_to_north: bool,
}

impl Traversed {
    fn new() -> Self {
        Traversed {
            east_to_west: false,
            west_to_east: false,
            north_to_south: false,
            south_to_north: false,
        }
    }

    fn add_direction_visited(&mut self, direction_to: Direction) {
        match direction_to {
            North => self.south_to_north = true,
            East => self.west_to_east = true,
            South => self.north_to_south = true,
            West => self.east_to_west = true,
        }
    }
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut obstructions_vert = HashMap::new();
    let mut obstructions_hor = HashMap::new();
    let mut guard_pos = (0, 0);
    let guard_direction = North;
    for (vert_pos, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        for (hor_pos, val) in line.chars().enumerate() {
            match val {
                '#' => {
                    obstructions_vert
                        .entry(vert_pos)
                        .or_insert(HashSet::new())
                        .insert(hor_pos);
                    obstructions_hor
                        .entry(hor_pos)
                        .or_insert(HashSet::new())
                        .insert(vert_pos);
                }
                '^' => {
                    guard_pos = (vert_pos, hor_pos);
                }
                _ => (),
            }
        }
    }

    let max_ver = *obstructions_vert.keys().max().unwrap();
    let max_hor = *obstructions_hor.keys().max().unwrap();

    let (count_p1, count_p2) = move_guard(
        guard_pos,
        guard_direction,
        &mut obstructions_vert,
        &mut obstructions_hor,
        max_ver,
        max_hor,
    );
    (count_p1, count_p2)
}

fn move_guard(
    mut guard_pos: MapIndex,
    mut guard_direction: Direction,
    obstructions_ver: &mut HashMap<usize, HashSet<usize>>,
    obstructions_hor: &mut HashMap<usize, HashSet<usize>>,
    max_ver: usize,
    max_hor: usize,
) -> (usize, usize) {
    let mut visited = HashMap::new();
    let mut traversed = Traversed::new();
    traversed.add_direction_visited(North);
    visited.insert(guard_pos, traversed);
    let mut already_blocked = HashSet::new();
    already_blocked.insert(guard_pos);
    let mut counter = 0;
    let mut iteration = 0;
    loop {
        // println!(
        //     "guard ver, guard hor {:?}, towards {guard_direction:?}",
        //     guard_pos
        // );
        if iteration > 4 {
            return (5, 5);
        } else {
            // iteration += 1;
        }
        debug_print_visited(
            &visited,
            &already_blocked,
            max_ver,
            max_hor,
            obstructions_hor,
            guard_pos,
        );
        let (next_pos, finished) = next_before_obstruction(
            guard_direction,
            obstructions_ver,
            obstructions_hor,
            guard_pos,
            max_ver,
            max_hor,
        );
        counter += patrol_virtual_guard(
            &visited,
            &mut already_blocked,
            obstructions_ver,
            obstructions_hor,
            next_pos,
            guard_pos,
            guard_direction,
            max_ver,
            max_hor,
        );
        insert_into_visited_inclusive(&mut visited, next_pos, guard_pos, guard_direction);
        if finished {
            break;
        }
        guard_pos = next_pos;
        guard_direction = guard_direction.next_direction();
    }

    debug_print_visited(
        &visited,
        &already_blocked,
        max_ver,
        max_hor,
        obstructions_hor,
        guard_pos,
    );
    (visited.len(), counter)
}

fn move_virtual_guard(
    visited: &HashMap<(usize, usize), Traversed>,
    obstructions_ver: &HashMap<usize, HashSet<usize>>,
    obstructions_hor: &HashMap<usize, HashSet<usize>>,
    mut guard_pos: MapIndex,
    mut guard_direction: Direction,
    max_ver: usize,
    max_hor: usize,
) -> bool {
    let mut virtual_visited = HashMap::new();

    let block_pos = guard_direction.next_pos(guard_pos);
    if obstructions_ver
        .get(&block_pos.0)
        .map(|hm| hm.contains(&guard_pos.1))
        .unwrap_or(false)
    {
        // Do nothing, there already is a block so it is part of the main path already
        return false;
    }
    guard_direction = guard_direction.next_direction();
    loop {
        let (next_pos, finished) = next_before_obstruction(
            guard_direction,
            obstructions_ver,
            obstructions_hor,
            guard_pos,
            max_ver,
            max_hor,
        );
        if finished {
            return false;
        }
        // check if we now loop into the visited path somehow
        if let Some(traverse) = visited.get(&next_pos) {
            if guard_direction.loops_at_intersect(traverse) {
                return true;
            }
        }

        if let Some(virtual_visited) = virtual_visited.get(&next_pos) {
            if guard_direction.loops_at_intersect(virtual_visited) {
                return true;
            }
        }

        insert_into_visited_inclusive(&mut virtual_visited, next_pos, guard_pos, guard_direction);
        guard_pos = next_pos;
        guard_direction = guard_direction.next_direction();
    }
}

fn next_before_obstruction(
    direction: Direction,
    obstructions_ver: &HashMap<usize, HashSet<usize>>,
    obstructions_hor: &HashMap<usize, HashSet<usize>>,
    guard_pos: MapIndex,
    ver_max: usize,
    hor_max: usize,
) -> (MapIndex, bool) {
    let obstructions = match direction {
        North => obstructions_hor
            .get(&guard_pos.1)
            .ok_or(((0, guard_pos.1), true)),
        South => obstructions_hor
            .get(&guard_pos.1)
            .ok_or(((ver_max, guard_pos.1), true)),
        West => obstructions_ver
            .get(&guard_pos.0)
            .ok_or(((guard_pos.0, 0), true)),
        East => obstructions_ver
            .get(&guard_pos.0)
            .ok_or(((guard_pos.0, hor_max), true)),
    };
    match obstructions {
        Ok(obstructions) => match direction {
            North => obstructions
                .iter()
                .filter(|&vert_pos| *vert_pos < guard_pos.0)
                .max()
                .map(|obs| ((obs + 1, guard_pos.1), false))
                .unwrap_or(((0, guard_pos.1), true)),
            South => obstructions
                .iter()
                .filter(|&vert_pos| *vert_pos > guard_pos.0)
                .min()
                .map(|obs| ((obs - 1, guard_pos.1), false))
                .unwrap_or(((ver_max, guard_pos.1), true)),
            West => obstructions
                .iter()
                .filter(|&hor_pos| *hor_pos < guard_pos.1)
                .max()
                .map(|obs| ((guard_pos.0, obs + 1), false))
                .unwrap_or(((guard_pos.0, 0), true)),
            East => obstructions
                .iter()
                .filter(|&hor_pos| *hor_pos > guard_pos.1)
                .min()
                .map(|obs| ((guard_pos.0, obs - 1), false))
                .unwrap_or(((guard_pos.0, hor_max), true)),
        },
        Err(res) => res,
    }
}

fn insert_into_visited_inclusive(
    visited: &mut HashMap<(usize, usize), Traversed>,
    next_pos: MapIndex,
    guard_pos: MapIndex,
    to_direction: Direction,
) {
    for ver_val in RangeInclusive::new(guard_pos.0.min(next_pos.0), guard_pos.0.max(next_pos.0)) {
        for hor_val in RangeInclusive::new(guard_pos.1.min(next_pos.1), guard_pos.1.max(next_pos.1))
        {
            // now add that we traversed this one
            visited
                .entry((ver_val, hor_val))
                .or_insert(Traversed::new())
                .add_direction_visited(to_direction);
        }
    }
}

fn patrol_virtual_guard(
    visited: &HashMap<MapIndex, Traversed>,
    already_blocked: &mut HashSet<MapIndex>,
    obstructions_ver: &HashMap<usize, HashSet<usize>>,
    obstructions_hor: &HashMap<usize, HashSet<usize>>,
    guard_pos: MapIndex,
    next_pos: MapIndex,
    to_direction: Direction,
    max_hor: usize,
    max_ver: usize,
) -> usize {
    let mut count_loops = 0;
    for ver_val in RangeInclusive::new(guard_pos.0.min(next_pos.0), guard_pos.0.max(next_pos.0)) {
        for hor_val in RangeInclusive::new(guard_pos.1.min(next_pos.1), guard_pos.1.max(next_pos.1))
        {
            let virt_guard_startpos = (ver_val, hor_val);
            if !to_direction.can_move(virt_guard_startpos, max_ver, max_hor) {
                continue;
            }
            if already_blocked.contains(&to_direction.next_pos(virt_guard_startpos)) {
                continue;
            }
            if move_virtual_guard(
                &visited,
                obstructions_ver,
                obstructions_hor,
                (ver_val, hor_val),
                to_direction,
                max_ver,
                max_hor,
            ) {
                already_blocked.insert(to_direction.next_pos(virt_guard_startpos));
                count_loops += 1;
            }
        }
    }
    count_loops
}

fn debug_print_visited(
    visited: &HashMap<MapIndex, Traversed>,
    already_blocked: &HashSet<MapIndex>,
    max_hor: usize,
    max_ver: usize,
    obstructions_hor: &HashMap<usize, HashSet<usize>>,
    guard_pos: MapIndex,
) {
    return;
    for ver in 0..=max_ver {
        for hor in 0..=max_hor {
            let default = HashSet::new();
            let obstructions_hor = obstructions_hor.get(&hor).unwrap_or(&default);
            if already_blocked.contains(&(ver, hor)) {
                print!("O")
            } else if let Some(trav) = visited.get(&(ver, hor)) {
                if ver == guard_pos.0 && hor == guard_pos.1 {
                    print!("@")
                } else if (trav.north_to_south || trav.south_to_north)
                    && (trav.west_to_east || trav.east_to_west)
                {
                    print!("+");
                } else if trav.north_to_south || trav.south_to_north {
                    print!("|");
                } else {
                    print!("-");
                }
            } else if obstructions_hor.contains(&ver) {
                print!("#");
            } else if ver == guard_pos.0 && hor == guard_pos.1 {
                print!("G")
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
