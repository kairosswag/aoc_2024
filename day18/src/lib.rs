use std::collections::BTreeSet;
use std::io::BufRead;

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

    let res_p2 = loop {
        let line = &lines.next().unwrap().unwrap();
        let (x, y) = get_xy(&line);
        walls[to_coord(x, y)] = true;
        let threshold = costing[to_coord(x, y)];
        if threshold == usize::MAX - 1 {
            // threshold not in shortest path
            continue;
        }
        costing[to_coord(x, y)] = usize::MAX - 1;
        wipe_costing(&mut costing, threshold, &mut to_visit);
        if let None = find_path(&walls, &mut costing, &mut to_visit) {
            break format!("{},{}", x, y);
        }
    };

    (res_p1, res_p2)
}

fn wipe_costing(costing: &mut [usize], threshold: usize, to_visit: &mut BTreeSet<(usize, (usize, usize))>) {
    while let Some(option) = to_visit.last() {
        if option.0 > threshold {
            to_visit.pop_last();
        } else {
            break;
        }
    }
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let val = &mut costing[to_coord(x, y)];
            if *val > threshold {
                *val = usize::MAX - 1;
            } else if *val == threshold {
                *val = usize::MAX - 1;
                to_visit.insert((threshold, (x, y)));
            }
        }
    }
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
