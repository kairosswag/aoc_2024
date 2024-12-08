use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::io::BufRead;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    ver: isize,
    hor: isize,
}

impl Coordinate {
    fn from(ver: isize, hor: isize) -> Coordinate {
        Coordinate { ver, hor }
    }
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut nodes = HashMap::new();
    let mut set = false;
    let mut max_ver = 0;
    let mut max_hor = 0;
    for (ver_pos, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if !set {
            max_hor = line.len();
            set = true;
        }
        max_ver = ver_pos;
        for (hor_pos, antenna) in line.chars().enumerate() {
            if antenna != '.' {
                nodes
                    .entry(antenna)
                    .or_insert_with(HashSet::new)
                    .insert(Coordinate::from(ver_pos as isize, hor_pos as isize));
            }
        }
    }
    max_ver += 1;

    let (anti_nodes, resonant_antinodes) = calc_antinodes(&nodes, max_ver as isize, max_hor as isize);

    (anti_nodes.len(), resonant_antinodes.len())
}

fn calc_antinodes(
    nodes: &HashMap<char, HashSet<Coordinate>>,
    max_ver: isize,
    max_hor: isize,
) -> (HashSet<Coordinate>, HashSet<Coordinate>) {
    let mut antinodes = HashSet::new();
    let mut resonant_antinodes = HashSet::new();
    for node in nodes.keys() {
        let coordinates = nodes.get(node).unwrap();
        for combination in coordinates.iter().combinations(2) {
            let first = combination[0];
            let second = combination[1];
            let delta = second - first;
            let anti_1 = second + &delta;
            let anti_2 = first - &delta;
            if is_in_bounds(&anti_1, max_ver, max_hor) {
                antinodes.insert(anti_1);
            }
            if is_in_bounds(&anti_2, max_ver, max_hor) {
                antinodes.insert(anti_2);
            }

            let mut curr_node = *first;
            while is_in_bounds(&curr_node, max_ver, max_hor) {
                resonant_antinodes.insert(curr_node);
                curr_node = &curr_node + &delta;
            }

            curr_node = first - &delta;
            while is_in_bounds(&curr_node, max_ver, max_hor) {
                resonant_antinodes.insert(curr_node);
                curr_node = &curr_node - &delta;
            }
        }
    }

    (antinodes, resonant_antinodes)
}

fn is_in_bounds(coordinate: &Coordinate, max_ver: isize, max_hor: isize) -> bool {
    coordinate.ver >= 0 && coordinate.ver < max_ver && coordinate.hor >= 0 && coordinate.hor < max_hor
}

impl Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            ver: self.ver - rhs.ver,
            hor: self.hor - rhs.hor,
        }
    }
}
impl Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            ver: self.ver + rhs.ver,
            hor: self.hor + rhs.hor,
        }
    }
}
