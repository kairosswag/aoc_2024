use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{

    let mut regex_groups: Vec<String> = Vec::new();
    let mut lines = reader.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        for expr in line.split(",").map(|s| s.trim()) {
            regex_groups.push(expr.trim().to_string());
        }
    }
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap().unwrap();
        if line.is_empty() {
            break;
        }

        let mut results_for_depth = vec![usize::MAX; line.len()];
        let possibilities = match_string_recursive(&line, &regex_groups, 0, &mut results_for_depth);
        if possibilities > 0 {
            count_p1 += 1;
            count_p2 += possibilities;
        }

    }

    (count_p1, count_p2)
    
}

fn match_string_recursive(remaining_string: &str, towels: &[String], curr_depth: usize, results_for_depth: &mut [usize]) -> usize {
    let remaining_len = remaining_string.len();
    if remaining_len == 0 {
        1
    } else if results_for_depth[curr_depth] != usize::MAX {
        results_for_depth[curr_depth]
    } else {
        let mut count = 0;
        for towel in towels {
            let towel_len = towel.len();
            if towel_len > remaining_len {
                continue;
            }
            if remaining_string.starts_with(towel) {
                count += match_string_recursive(&remaining_string[towel_len..], towels, curr_depth + towel_len, results_for_depth);
            }
        }
        results_for_depth[curr_depth] = count;
        count
    }
}



