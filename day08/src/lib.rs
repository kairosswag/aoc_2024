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

    println!("max_ver {}, max_hor {}", max_ver, max_hor);

    let anti_nodes = calc_antinodes(&nodes, max_ver as isize, max_hor as isize);

    for ver in 0..max_ver {
        for hor in 0..max_hor {
            if anti_nodes.get(&Coordinate::from(ver as isize, hor as isize)).is_some() {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n");
    }

    (anti_nodes.len(), 5)
}

fn calc_antinodes(
    nodes: &HashMap<char, HashSet<Coordinate>>,
    max_ver: isize,
    max_hor: isize,
) -> HashSet<Coordinate> {
    let mut antinodes = HashSet::new();
    for node in nodes.keys() {
        let coordinates = nodes.get(node).unwrap();
        for combination in coordinates.iter().combinations(2) {
            let first = combination[0];
            let second = combination[1];
            let delta = second - first;
            let anti_1 = second + &delta;
            let anti_2 = first - &delta;
            if *node == 'A' {
                println!("returning values for a: {:?} {:?}", anti_1, anti_2);
            }
            if anti_1.ver >= 0 && anti_1.ver < max_ver && anti_1.hor >= 0 && anti_1.hor < max_hor {
                if *node == 'A' {
                    println!("Insertinging anti_1: {:?}", anti_1);
                }
                antinodes.insert(anti_1);
            }
            if anti_2.ver >= 0 && anti_2.ver < max_ver && anti_2.hor >= 0 && anti_2.hor < max_hor {
                if *node == 'A' {
                    println!("Insertinging anti_2: {:?}", anti_2);
                }
                antinodes.insert(anti_2);
            }
        }
    }

    antinodes
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
