use hashbrown::{HashMap, HashSet};
use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut po_rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut updates: Vec<Vec<u8>> = Vec::new();
    let mut read_rules = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            read_rules = false;
            continue;
        }

        if read_rules {
            let first_rule = line[0..2].parse::<u8>().unwrap();
            let second_rule = line[3..5].parse::<u8>().unwrap();
            po_rules.entry(second_rule).or_insert(Vec::new()).push(first_rule);
        } else {
            updates.push(line.split(",").map(|s| s.parse::<u8>().unwrap()).collect());
        }
    }

    let mut count_p1: usize = 0;
    'outer: for update in &updates {
        let mut forbidden = HashSet::new();
        for page in update {
            if forbidden.contains(&page) {
                continue 'outer;
            } else {
                if let Some(followers) = po_rules.get(page) {
                    for follower in followers {
                        forbidden.insert(follower);
                    }
                }
            }
        }
        count_p1 += update[update.len() / 2] as usize;
    }

    let count_p2 = 5;

    (count_p1, count_p2)
}

