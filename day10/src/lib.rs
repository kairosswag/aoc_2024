use hashbrown::HashSet;
use std::io::BufRead;

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut buf = [0u8; 2usize.pow(16)];
    let read = reader.read(&mut buf).unwrap();

    let mut width = 0;
    while buf[width] != b'\n' {
        width += 1;
    }
    let height = read / width;

    let mut initial = HashSet::new();
    for hor_val in 0..width {
        for ver_val in 0..=height {
            let idx = ver_val * (width + 1) + hor_val;
            if buf[idx] == b'0' {
                initial.insert((ver_val, hor_val));
            }
        }
    }

    let mut count = 0;
    for init in &initial {
        let mut curr_round = HashSet::new();
        curr_round.insert(*init);
        let mut next_round = HashSet::new();
        let mut curr_val = b'0';
        loop {
            for (ver, hor) in &curr_round {
                for (neighbor_ver, neighbor_hor) in
                    valid_neighbors(*ver, *hor, curr_val, &buf, width, height)
                {
                    next_round.insert((neighbor_ver, neighbor_hor));
                }
            }

            if curr_val == b'8' {
                count += next_round.len();
                break;
            } else {
                curr_round = next_round;
                next_round = HashSet::new();
                curr_val += 1;
            }
        }
    }

    let mut count_p2 = 0;
    for init in &initial {
        let mut curr_round = Vec::new();
        curr_round.push(*init);
        let mut next_round = Vec::new();
        let mut curr_val = b'0';
        loop {
            for (ver, hor) in &curr_round {
                for (neighbor_ver, neighbor_hor) in
                    valid_neighbors(*ver, *hor, curr_val, &buf, width, height)
                {
                    next_round.push((neighbor_ver, neighbor_hor));
                }
            }

            if curr_val == b'8' {
                count_p2 += next_round.len();
                break;
            } else {
                curr_round = next_round;
                next_round = Vec::new();
                curr_val += 1;
            }
        }
    }
    (count, count_p2)
}

fn valid_neighbors(
    ver: usize,
    hor: usize,
    curr_val: u8,
    map: &[u8],
    max_width: usize,
    max_height: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if ver > 0 {
        let neighbor_val = map[get_idx(ver - 1, hor, max_width)];
        if neighbor_val == curr_val + 1 {
            neighbors.push((ver - 1, hor));
        }
    }
    if hor > 0 {
        let neighbor_val = map[get_idx(ver, hor - 1, max_width)];
        if neighbor_val == curr_val + 1 {
            neighbors.push((ver, hor - 1));
        }
    }
    if ver < max_height {
        let neighbor_val = map[get_idx(ver + 1, hor, max_width)];
        if neighbor_val == curr_val + 1 {
            neighbors.push((ver + 1, hor));
        }
    }
    if hor < max_width {
        let neighbor_val = map[get_idx(ver, hor + 1, max_width)];
        if neighbor_val == curr_val + 1 {
            neighbors.push((ver, hor + 1));
        }
    }
    neighbors
}

fn get_idx(ver: usize, hor: usize, width: usize) -> usize {
    ver * (width + 1) + hor
}
