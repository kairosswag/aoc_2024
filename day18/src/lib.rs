use std::collections::BTreeSet;
use std::io::BufRead;
use hashbrown::HashSet;

const WIDTH: usize = 70usize;
const HEIGHT: usize = 70usize;
pub fn run<R>(reader: R) -> (usize, String)
where
    R: BufRead,
{
    let mut costing = [usize::MAX - 1; (WIDTH + 1) * (HEIGHT + 1)];
    let mut walls = [false; (WIDTH + 1) * (HEIGHT + 1)];
    let mut lines = reader.lines();
    for _ in 0..1024 {
        let line = lines.next().unwrap().unwrap();
        let (x, y) = get_xy(&line);
        walls[to_coord(x, y)] = true;
    }

    let mut to_visit = BTreeSet::new();
    to_visit.insert((0, (0, 0)));
    let res_p1 = find_path(&walls, &mut costing, &mut to_visit).unwrap();

    let mut following_walls = Vec::new();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap().unwrap();
        let (x, y) = get_xy(&line);
        walls[to_coord(x, y)] = true;
        following_walls.push((x, y));
    }

    let mut colors = [0; (WIDTH + 1) * (HEIGHT + 1)];
    flood_fill(&mut colors, WIDTH, HEIGHT, 5999, &walls);
    flood_fill(&mut colors, 0, 0, 1, &walls);
    let mut color = 2;
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let coord = to_coord(x, y);
            if !walls[coord] && colors[coord] == 0 {
                flood_fill(&mut colors, x, y, color, &walls);
                color += 1;
            }
        }
    }
    let mut res_p2 = None;
    let mut point_to = vec![u16::MAX; 6000];
    while let Some((x, y)) = following_walls.pop() {
        walls[to_coord(x, y)] = false;
        let n_colors: Vec<u16> = fill_neighbors(x, y, &walls)
                .iter()
                .map(|(x, y)| colors[to_coord(*x, *y)]).collect();
        let unified_color = if n_colors.is_empty() {
            let new_color = color;
            color += 1;
            new_color
        } else {
            unify(n_colors, &mut point_to)
        };
        colors[to_coord(x, y)] = unified_color;
        if start_end_unified(&point_to) {
            res_p2 = Some(format!("{x},{y}"));
            break;
        }
    }
    let res_p2 = res_p2.unwrap();
    (res_p1, res_p2)
}

fn start_end_unified(point_to: &Vec<u16>) -> bool {
    let mut prev_pointee = 1;
    let mut curr_pointee = point_to[1];
    while curr_pointee != u16::MAX && prev_pointee != curr_pointee {
        if curr_pointee == 5999 {
            return true;
        }
        prev_pointee = curr_pointee;
        curr_pointee = point_to[curr_pointee as usize];
    }
    false
}

fn unify(mut unify_set: Vec<u16>, point_to: &mut Vec<u16>) -> u16 {
    let mut all_colors = Vec::new();

    while let Some(color_base) = unify_set.pop() {
        all_colors.push(color_base);
        let pointing_to = point_to[color_base as usize];
        if pointing_to != u16::MAX && pointing_to != color_base {
            unify_set.push(pointing_to);
        }
    }

    let max = all_colors.iter().max().unwrap().clone();
    for color in all_colors {
        point_to[color as usize] = max;
    }
    max
}

fn flood_fill(
    colors: &mut [u16; 5041],
    start_x: usize,
    start_y: usize,
    color: u16,
    walls: &[bool; 5041],
) {
    let mut neighbors = Vec::new();
    neighbors.push((start_x, start_y));
    let mut visited = HashSet::new();
    while let Some((x, y)) = neighbors.pop() {
        visited.insert((x, y));
        colors[to_coord(x, y)] = color;
        for (n_x, n_y) in fill_neighbors(x, y, walls) {
            if !visited.contains(&(n_x, n_y)) {
                neighbors.push((n_x, n_y));
            }
        }
    }
}

fn fill_neighbors(x: usize, y: usize, walls: &[bool; 5041]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let can_visit = |x, y| {
        let pos = to_coord(x, y);
        !walls[pos]
    };
    if x > 0 && can_visit(x - 1, y) {
        neighbors.push((x - 1, y));
    }
    if x < WIDTH && can_visit(x + 1, y) {
        neighbors.push((x + 1, y));
    }
    if y > 0 && can_visit(x, y - 1) {
        neighbors.push((x, y - 1));
    }
    if y < HEIGHT && can_visit(x, y + 1) {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn get_xy(line: &str) -> (usize, usize) {
    let (x, y) = line.split_once(',').unwrap();
    let x = x.parse::<usize>().unwrap();
    let y = y.parse::<usize>().unwrap();

    (x, y)
}

fn find_path(
    walls: &[bool],
    costing: &mut [usize],
    to_visit: &mut BTreeSet<(usize, (usize, usize))>,
) -> Option<usize> {
    while let Some((cost, (x, y))) = to_visit.pop_first() {
        costing[to_coord(x, y)] = cost;
        if x == WIDTH && y == HEIGHT {
            return Some(cost);
        }

        for (neighbor_x, neighbor_y) in neighbors(walls, costing, x, y) {
            to_visit.insert((cost + 1, (neighbor_x, neighbor_y)));
        }
    }

    None
}

fn neighbors(walls: &[bool], costing: &[usize], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let can_visit = |x, y| {
        let pos = to_coord(x, y);
        !walls[pos] && costing[pos] == usize::MAX - 1
    };
    if x > 0 && can_visit(x - 1, y) {
        neighbors.push((x - 1, y));
    }
    if x < WIDTH && can_visit(x + 1, y) {
        neighbors.push((x + 1, y));
    }
    if y > 0 && can_visit(x, y - 1) {
        neighbors.push((x, y - 1));
    }
    if y < HEIGHT && can_visit(x, y + 1) {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn to_coord(x: usize, y: usize) -> usize {
    y * (WIDTH + 1) + x
}
