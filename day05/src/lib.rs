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
            let mut forbidden_stack = vec![HashSet::new()];
            let mut maybe_ordered = Vec::new();
            for page in update {
                flansch(&mut forbidden_stack, page, &mut maybe_ordered, &po_rules);
            }
            count_p2 += maybe_ordered[maybe_ordered.len() / 2] as usize
        }
    }
    (count_p1, count_p2)
}

fn flansch(forbidden_stack: &mut Vec<HashSet<u8>>, page: &u8, maybe_ordered: &mut Vec<u8>, po_rules: &HashMap<u8, Vec<u8>>) {
    // println!("Curr page: {page}");
    let mut helper_stack = Vec::new();
    while forbidden_stack.last().unwrap().contains(page) {
        forbidden_stack.pop();
        let minus = maybe_ordered.pop().unwrap();
        helper_stack.push(minus);
        // println!("{:?} - {minus}", maybe_ordered);
    }
    maybe_ordered.push(*page);
    let mut new_forbidden = forbidden_stack.last().unwrap().clone();
    if let Some(followers) = po_rules.get(page) {
        for follower in followers {
            new_forbidden.insert(*follower);
        }
    }
    forbidden_stack.push(new_forbidden);
    // println!("Helper: {:?}", helper_stack);
    for idx in 0..helper_stack.len() {
        flansch(forbidden_stack, &helper_stack[idx], maybe_ordered, po_rules);
        // println!("{:?} + {val}", maybe_ordered);
    }
    // println!("End: {:?}", maybe_ordered);
}

