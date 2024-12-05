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
    let mut count_p2 = 0;
    for update in &updates {
        let mut forbidden = HashSet::new();
        let mut valid = true;
        for page in update {
            if forbidden.contains(&page) {
                valid = false;
                break;
            } else {
                if let Some(followers) = po_rules.get(page) {
                    for follower in followers {
                        forbidden.insert(follower);
                    }
                }
            }
        }
        if valid {
            count_p1 += update[update.len() / 2] as usize;
        } else {
            count_p2 += calc_specialized_total_order(&po_rules, &update);
        }
    }
    (count_p1, count_p2)
}

fn calc_specialized_total_order(po_rules: &HashMap<u8, Vec<u8>>, update_line: &Vec<u8>) -> usize {
    let mut pages: HashMap<&u8, usize> = update_line.iter().map(|val| (val, 0)).collect();
    for page in update_line {
        if let Some(followers) = po_rules.get(page) {
            for follower in followers {
                if let Some(val) = pages.get_mut(follower) {
                    *val += 1;
                }
            }
        }
    }

    let len = update_line.len();
    let mut curr = 0;
    while !pages.is_empty() {
        let remove = pages.iter().find(|val| *val.1 == 0).expect("whelp, that didn't work");
        let removed: u8 = **remove.0;
        pages.remove(&removed);
        if let Some(followers) = po_rules.get(&removed) {
            for follower in followers {
                if let Some(val) = pages.get_mut(follower) {
                    *val -= 1;
                }
            }
        }

        if curr == len / 2 {
            return removed as usize;
        } else {
            curr += 1;
        }
    }
    unreachable!()

}

