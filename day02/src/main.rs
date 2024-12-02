use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    for line in reader.lines() {
        let res: Vec<i32> = line
            .expect("line parse failed")
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        let mut ascending = 0;
        if res[0] < res[1] {
            ascending += 1;
        } else {
            ascending -= 1;
        }
        if res[1] < res[2] {
            ascending += 1;
        } else {
            ascending -= 1;
        }
        if res[2] < res[3] {
            ascending += 1;
        } else {
            ascending -= 1;
        }

        let skip_list =  valid_p1(&res, ascending > 0);
        if skip_list.is_empty(){
            count_p1 += 1;
            count_p2 += 1;
        } else if valid_p2(&res, ascending > 0, skip_list) {
            count_p2 += 1;
        }
    }
    (count_p1, count_p2)
}

fn valid_p2(list: &[i32], line_ascending: bool, skip_list: Vec<usize>) -> bool {
    if list.len() == 3 {
        panic!("wut")
    }
    'skip: for skip_idx in skip_list {
        for i in 0..list.len() - 1 {
            let a = i;
            let mut b = i+1;
            if a == skip_idx {
                continue;
            }
            if b == skip_idx {
                b += 1;
                if b == list.len() {
                    return true; // we can safely remove the last element
                }
            }
            if !valid_when(list[a], list[b], line_ascending) {
               continue 'skip
            }
        }
        // println!("{line: } safe when removing {skip_idx} / {}", list[skip_idx]);
        return true;
    }
    false
}

fn valid_p1(list: &[i32], line_ascending: bool) -> Vec<usize> {
    let mut error_idx = HashSet::new();
    for i in 0..list.len() - 1 {
        if !valid_when(list[i], list[i + 1], line_ascending) {
            error_idx.insert(i);
            error_idx.insert(i + 1);
        }
    }
    error_idx.into_iter().collect()
}

fn valid_when(a: i32, b: i32, ascending: bool) -> bool {
    let val = b - a;
    let is_ascending = val > 0;
    if is_ascending != ascending {
        return false;
    }
    val.abs() <= 3 && val != 0
}


#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!((2, 4), run(input.as_bytes()));
    }
}
