use crate::Facing::{Horizontal, Vertical};
use hashbrown::{HashMap, HashSet};
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct MazeSpot {
    pub position: Coordinate,
    pub facing: Facing,
}

impl MazeSpot {
    fn new(x: usize, y: usize, facing: Facing) -> MazeSpot {
        MazeSpot {
            position: Coordinate::new(x, y),
            facing,
        }
    }

    fn get_moves(&self) -> Vec<(MazeSpot, usize)> {
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
    let mut reindeer_maze = HashMap::new();
    let mut start = None;
    let mut end = None;
    for (y_val, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x_val, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    reindeer_maze.insert(MazeSpot::new(x_val, y_val, Horizontal), usize::MAX);
                    reindeer_maze.insert(MazeSpot::new(x_val, y_val, Vertical), usize::MAX);
                }
                'S' => {
                    start = Some(MazeSpot::new(x_val, y_val, Horizontal));
                    reindeer_maze.insert(MazeSpot::new(x_val, y_val, Vertical), usize::MAX);
                }
                'E' => {
                    end = Some(Coordinate::new(x_val, y_val));
                    reindeer_maze.insert(MazeSpot::new(x_val, y_val, Horizontal), usize::MAX);
                    reindeer_maze.insert(MazeSpot::new(x_val, y_val, Vertical), usize::MAX);
                }
                _ => (),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let (res_p1, predecessors) = determine_cost(start, end, &mut reindeer_maze);

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
    reindeer_maze: &mut HashMap<MazeSpot, usize>,
) -> (usize, HashMap<MazeSpot, Vec<MazeSpot>>) {
    reindeer_maze.insert(
        MazeSpot::new(start.position.x, start.position.y, Vertical),
        usize::MAX,
    );
    let mut visited = HashSet::new();
    let mut predecessors = HashMap::new();
    reindeer_maze.insert(start, 0);
    loop {
        let spot = {
            let val = reindeer_maze.iter().min_by_key(|(_, cost)| *cost);
            let (spot, cost) = val.unwrap();
            if *cost == usize::MAX {
                unreachable!("found nothing :/");
            }
            if spot.position == end {
                return (*cost, predecessors);
            }
            visited.insert(spot.clone());
            spot.clone()
        };

        let cost = reindeer_maze.remove(&spot).unwrap();

        for (potential_move, move_cost) in spot.get_moves() {
            if visited.contains(&potential_move) {
                continue;
            }
            if let Some(curr_dist) = reindeer_maze.get_mut(&potential_move) {
                if *curr_dist > cost + move_cost {
                    *curr_dist = cost + move_cost;

                    if potential_move.position == end {
                        println!("adding end to followers {:?}, {potential_move:?}", spot.clone());
                    }
                    predecessors.insert(potential_move, vec![spot.clone()]);

                } else if *curr_dist == cost + move_cost {
                    predecessors.get_mut(&potential_move).unwrap().push(spot.clone());
                }
            }
        }
    }
}
