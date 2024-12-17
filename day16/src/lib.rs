use crate::Facing::{Horizontal, Vertical};
use hashbrown::{HashMap, HashSet};
use std::collections::BTreeSet;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Facing {
    Horizontal,
    Vertical,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
struct MazeSpot {
    pub position: Coordinate,
    pub facing: Facing,
}

struct MazeMap {
    hor_maze: Vec<u32>,
    ver_maze: Vec<u32>,
    width: usize,
}

impl MazeMap {
    fn new(hor_maze: Vec<u32>, ver_maze: Vec<u32>, width: usize) -> MazeMap {
        MazeMap {
            hor_maze,
            ver_maze,
            width,
        }
    }

    fn get_mut(&mut self, spot: &MazeSpot) -> &mut u32 {
        match spot.facing {
            Horizontal => &mut self.hor_maze[spot.position.y * self.width + spot.position.x],
            Vertical => &mut self.ver_maze[spot.position.y * self.width + spot.position.x],
        }
    }
}

impl MazeSpot {
    fn new(x: usize, y: usize, facing: Facing) -> MazeSpot {
        MazeSpot {
            position: Coordinate::new(x, y),
            facing,
        }
    }

    fn get_moves(&self) -> Vec<(MazeSpot, u32)> {
        let x_pos = self.position.x;
        let y_pos = self.position.y;
        let heading = self.facing;

        let mut moves = Vec::new();
        match heading {
            Horizontal => {
                moves.push((Self::new(x_pos - 1, y_pos, heading), 1));
                moves.push((Self::new(x_pos + 1, y_pos, heading), 1));
                moves.push((Self::new(x_pos, y_pos, Vertical), 1000));
            }
            Vertical => {
                moves.push((Self::new(x_pos, y_pos - 1, heading), 1));
                moves.push((Self::new(x_pos, y_pos + 1, heading), 1));
                moves.push((Self::new(x_pos, y_pos, Horizontal), 1000));
            }
        }

        moves
    }

}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut horizontal_maze = vec![u32::MAX; 141 * 141];
    let mut vertical_maze = vec![u32::MAX; 141 * 141];
    let mut start = None;
    let mut end = None;
    let mut width = 0;
    for (y_val, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        width = line.len();
        for (x_val, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    horizontal_maze[y_val * width + x_val] = u32::MAX - 1;
                    vertical_maze[y_val * width + x_val] = u32::MAX - 1;
                }
                'S' => {
                    start = Some(MazeSpot::new(x_val, y_val, Horizontal));
                    horizontal_maze[y_val * width + x_val] = 0;
                    vertical_maze[y_val * width + x_val] = u32::MAX - 1;
                }
                'E' => {
                    end = Some(Coordinate::new(x_val, y_val));
                    horizontal_maze[y_val * width + x_val] = u32::MAX - 1;
                    vertical_maze[y_val * width + x_val] = u32::MAX - 1;
                }
                _ => (),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let maze = MazeMap::new(horizontal_maze, vertical_maze, width);
    let (res_p1, predecessors) = determine_cost(start, end, maze);

    let mut in_path = HashSet::new();
    let mut waiting = Vec::new();
    waiting.push(MazeSpot::new(end.x, end.y, Horizontal));
    waiting.push(MazeSpot::new(end.x, end.y, Vertical));

    while let Some(spot) = waiting.pop() {
        if in_path.insert(spot.clone()) {
            if let Some(predecessors) = predecessors.get(&spot) {
                for predecessor in predecessors {
                    waiting.push(predecessor.clone());
                }
            }
        }
    }

    let res_p2 = HashSet::<Coordinate>::from_iter(in_path.iter().map(|val| val.position)).len();

    (res_p1 as usize, res_p2)
}

fn determine_cost(
    start: MazeSpot,
    end: Coordinate,
    mut maze: MazeMap,
) -> (u32, HashMap<MazeSpot, Vec<MazeSpot>>) {
    let mut predecessors = HashMap::new();
    let mut min_heap = BTreeSet::new();
    min_heap.insert((0, start));
    loop {
        let (cost, spot) = min_heap.pop_first().unwrap();

        if cost == u32::MAX {
            unreachable!("found nothing :/");
        }
        if spot.position == end {
            return (cost, predecessors);
        }

        for (potential_move, move_cost) in spot.get_moves() {
            let curr_dist = maze.get_mut(&potential_move);
            if *curr_dist != u32::MAX {
                if *curr_dist > cost + move_cost {
                    min_heap.remove(&(*curr_dist, potential_move.clone()));
                    *curr_dist = cost + move_cost;
                    min_heap.insert((*curr_dist, potential_move.clone()));
                    predecessors.insert(potential_move, vec![spot.clone()]);
                } else if *curr_dist == cost + move_cost {
                    predecessors
                        .get_mut(&potential_move)
                        .unwrap()
                        .push(spot.clone());
                }
            }
        }
    }
}
