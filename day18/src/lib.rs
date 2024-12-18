use std::collections::BTreeSet;
use hashbrown::{HashMap, HashSet};
use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    
    let width = 71usize;
    let height = 71usize;
    let mut map = vec![usize::MAX - 1; width * height];

    for line in reader.lines().take(1024) {
        let line = line.unwrap();
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();


        map[y * width + x] = usize::MAX;
    }


    let res_p1 = find_path(&mut map, width, height);

    (res_p1, 5)

}

fn find_path(map: &mut [usize], width: usize, height: usize) -> usize {
    map[0] = 0;
    let mut to_visit = BTreeSet::new();
    to_visit.insert((0, (0, 0)));
    while let Some((cost, (x, y))) = to_visit.pop_first() {
        if x == width - 1 && y == height - 1 {
            return cost;
        }

        for (neighbor_x, neighbor_y) in neighbors(map, x, y, width, height) {
            to_visit.insert((cost + 1, (neighbor_x, neighbor_y)));
        }

    }

    panic!("no path found");
}

fn neighbors(map: &[usize], x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let can_visit = |x, y| map[y * width + x] == usize::MAX - 1;
    if x > 0 && can_visit(x - 1, y) {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 && can_visit(x + 1, y) {
        neighbors.push((x + 1, y));
    }
    if y > 0 && can_visit(x, y - 1) {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 && can_visit(x, y + 1) {
        neighbors.push((x, y + 1));
    }


    neighbors
}




