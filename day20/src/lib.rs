use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::Read;

fn valid_cheats(max_shortcuts: i32) -> Vec<(i32, i32, i32)> {
    let mut cheats = Vec::new();

    for x in 0..=max_shortcuts {
        for y in 0..=max_shortcuts {
            if x == 0 && y == 0 {
                continue;
            }
            if x + y > max_shortcuts {
                continue;
            }
            let skip_len = x + y;

            cheats.push((x, y, skip_len));
            cheats.push((-x, y, skip_len));
            cheats.push((x, -y, skip_len));
            cheats.push((-x, -y, skip_len));
        }
    }

    cheats.into_iter().unique().collect()
}

#[derive(Clone)]
struct Maze {
    layout: Vec<u16>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(layout: Vec<u16>, width: usize, height: usize) -> Maze {
        Maze {
            layout,
            width,
            height,
        }
    }

    fn val(&self, (x, y): (usize, usize)) -> u16 {
        self.layout[y * self.width + x]
    }

    fn val_mut(&mut self, (x, y): (usize, usize)) -> &mut u16 {
        &mut self.layout[y * self.width + x]
    }

    fn valid_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let can_visit = |x, y| self.val((x, y)) == u16::MAX - 1;
        if x > 0 && can_visit(x - 1, y) {
            neighbors.push((x - 1, y));
        }
        if x < self.width && can_visit(x + 1, y) {
            neighbors.push((x + 1, y));
        }
        if y > 0 && can_visit(x, y - 1) {
            neighbors.push((x, y - 1));
        }
        if y < self.height && can_visit(x, y + 1) {
            neighbors.push((x, y + 1));
        }

        neighbors
    }
}

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: Read,
{
    let (maze, start, end) = generate_maze(&mut reader);
    // print_maze(&maze, start, end);
    let maze_from_start = go_dutchman(maze.clone(), start);
    let maze_from_end = go_dutchman(maze, end);
    let path_len = maze_from_start.val(end);

    let res_p1 = find_shortcuts(&maze_from_start, &maze_from_end, path_len);
    let res_p2 = find_shortcuts_creatively(&maze_from_start, &maze_from_end, path_len, 20);

    (res_p1, res_p2)
}

fn generate_maze<R>(mut reader: R) -> (Maze, (usize, usize), (usize, usize))
where
    R: Read,
{
    let mut map = Vec::new();
    let mut buff = [0; 64];
    let mut len = 0;
    let mut len_set = false;
    let mut height = 0;
    let mut start = None;
    let mut end = None;
    let mut last_was_linebreak = false;
    while let Ok(val) = reader.read(&mut buff) {
        for idx in 0..val {
            last_was_linebreak = false;
            match buff[idx] {
                b'\n' => {
                    if !len_set {
                        len += idx;
                        len_set = true;
                    }
                    height += 1;
                    last_was_linebreak = true;

                }
                b'#' => map.push(u16::MAX),
                b'S' => {
                    let start_pos = map.len();
                    let start_x = start_pos % len;
                    let start_y = start_pos / len;
                    start = Some((start_x, start_y));
                    map.push(u16::MAX - 1);
                }
                b'E' => {
                    let end_pos = map.len();
                    let end_x = end_pos % len;
                    let end_y = end_pos / len;
                    end = Some((end_x, end_y));
                    map.push(u16::MAX - 1);
                }
                b'.' => {
                    map.push(u16::MAX - 1);
                }
                _ => unreachable!("could not resolve {}", buff[idx]),
            }
        }
        if !len_set {
            len += val
        }
        if val < buff.len() {
            break;
        }
    }
    if !last_was_linebreak {
        height += 1;
    }
    let start = start.unwrap();
    let end = end.unwrap();
    (Maze::new(map, len, height), start, end)
}

fn find_shortcuts_creatively(
    start_maze: &Maze,
    end_maze: &Maze,
    path_len: u16,
    max_shortcuts: usize,
) -> usize {
    let valid_cheats = valid_cheats(max_shortcuts as i32);
    let mut num_shortcuts = 0;
    for y in 0..start_maze.height {
        for x in 0..start_maze.width {
            let walk_to_distance = start_maze.val((x, y));
            if walk_to_distance == u16::MAX || walk_to_distance == u16::MAX - 1 {
                continue;
            }
            for (cheat_x, cheat_y, skip_len) in &valid_cheats {
                let i_x = x as i32 + cheat_x;
                let i_y = y as i32 + cheat_y;
                let valid_x = i_x >= 0 && i_x < start_maze.width as i32;
                let valid_y = i_y >= 0 && i_y < start_maze.height as i32;

                if valid_x && valid_y {
                    let nb_val = end_maze.val((i_x as usize, i_y as usize));
                    // wall, not initialized (i.e. enclave), not diff enough
                    if nb_val != u16::MAX
                        && nb_val != u16::MAX-1
                        && (walk_to_distance + *skip_len as u16) + (nb_val) + 99 <= path_len
                    {
                        num_shortcuts += 1;
                    }
                }
            }
        }
    }
    num_shortcuts
}

fn find_shortcuts(start_maze: &Maze, end_maze: &Maze, path_len: u16) -> usize {
    let mut num_shortcuts = 0;
    for y in 0..start_maze.height {
        for x in 0..start_maze.width {
            let walk_to_distance = start_maze.val((x, y));
            if walk_to_distance == u16::MAX || walk_to_distance == u16::MAX - 1 {
                continue;
            }
            let shortcuts = |x, y, skip_len| {
                let nb_val = end_maze.val((x, y));
                // wall, not initialized (i.e. enclave), not diff enough
                nb_val != u16::MAX
                    && nb_val != u16::MAX - 1
                    && (walk_to_distance + skip_len) + (nb_val) + 99 <= path_len
            };
            if x > 0 && shortcuts(x - 1, y, 1) {
                num_shortcuts += 1;
            }
            if x > 1 && shortcuts(x - 2, y, 2) {
                num_shortcuts += 1;
            }

            if x < start_maze.width - 1 && shortcuts(x + 1, y, 1) {
                num_shortcuts += 1;
            }
            if x < start_maze.width - 2 && shortcuts(x + 2, y, 2) {
                num_shortcuts += 1;
            }

            if y > 0 && shortcuts(x, y - 1, 1) {
                num_shortcuts += 1;
            }
            if y > 1 && shortcuts(x, y - 2, 2) {
                num_shortcuts += 1;
            }

            if y < start_maze.height - 1 && shortcuts(x, y + 1, 1) {
                num_shortcuts += 1;
            }
            if y < start_maze.height - 2 && shortcuts(x, y + 2, 2) {
                num_shortcuts += 1;
            }
        }
    }
    num_shortcuts
}


fn go_dutchman(mut maze: Maze, from: (usize, usize)) -> Maze {
    let mut min_heap = BTreeSet::new();
    min_heap.insert((0, from));
    *maze.val_mut(from) = 0;
    while let Some((cost, spot)) = min_heap.pop_first() {
        if cost == u16::MAX {
            unreachable!("sanity check");
        }

        for neighbor in maze.valid_neighbors(spot) {
            *maze.val_mut(neighbor) = cost + 1;
            min_heap.insert((cost + 1, neighbor));
        }
    }
    maze
}
